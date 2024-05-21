pub mod errors;

/// Contains functions used by the nodal analysis 
/// elements to calculate elemental flux between nodes.
pub mod flux_formulas;

pub mod ssdc_circuits;

// Standard modules
use std::collections::HashMap;
use std::{fmt::Debug, marker::PhantomData};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 3rd party modules
use serde::Serialize;
use gmatlib::{col_vec, Matrix};
use geqslib::newton::multivariate_newton_raphson;

// Local modules
use errors::{DroppedNodeError, EquationGenerationError, FluxCalculationError};
use flux_formulas::*;

/// TODO: Put a LOT of documentation here
pub type ElementConstructor<T> = fn (Weak<RefCell<GenericNode>>, Weak<RefCell<GenericNode>>, T) -> anyhow::Result<Rc<GenericElement>>;

/// A struct representing an element in a nodal analysis problem.
/// 
/// # Concept:
/// Elements are conduits for some "flux" value between nodes in a 
/// nodal analysis problem. In DC Circuits models, resistors, voltage 
/// sources, and current sources are examples of elements as they 
/// allow current (i.e. flux) to "travel" between two nodes. The way in
/// which this flux value is calculated may vary wildly between elements,
/// but **must** operate only knowing which nodes they connect, their
/// own gain value (e.g. resistance, voltage, and current, respectively in this case), 
/// and which node's potential they drive, if they drive one at all.
#[derive(Clone, Debug)]
pub struct GenericElement
{
    gain: Matrix<f64>,
    input_node: Weak<RefCell<GenericNode>>,
    output_node: Weak<RefCell<GenericNode>>,
    flux_calc: FluxCalculation,
    drives_output: bool,
}
impl GenericElement
{
    pub fn try_new(gain: Matrix<f64>,
        input_node: Weak<RefCell<GenericNode>>,
        output_node: Weak<RefCell<GenericNode>>,
        flux_calc: FluxCalculation,
        drives_output: bool,
        connect_to_input: bool,
        connect_to_output: bool,
    ) -> anyhow::Result<Rc<GenericElement>>
    {
        let elem = Rc::new(
            GenericElement { 
                gain, 
                input_node: Weak::clone(&input_node), 
                output_node: Weak::clone(&output_node), 
                flux_calc, 
                drives_output 
            }
        );

        // Make nodes aware of element
        if let (Some(input_refcell), Some(output_refcell)) = (input_node.upgrade(), output_node.upgrade())
        {
            if connect_to_input
            {
                input_refcell.try_borrow_mut()?.outputs.push(Rc::clone(&elem));
            }

            if connect_to_output
            {
                output_refcell.try_borrow_mut()?.inputs.push(Rc::clone(&elem));
            }

            Ok(elem)
        }
        else 
        {
            Err(DroppedNodeError.into())
        }
    }

    fn get_flux(&self) -> anyhow::Result<Matrix<f64>>
    {
        if let (Some(inode), Some(onode)) = (self.input_node.upgrade(), self.output_node.upgrade())
        {
            Ok((self.flux_calc)(inode, onode, &self.gain, self.drives_output)?)
        }
        else
        {
            Err(FluxCalculationError::NodeRefsAlreadyDropped.into())
        }
    }
}

#[derive(Clone, Debug)]
pub struct GenericNode
{
    potential: Matrix<f64>,
    inputs: Vec<Rc<GenericElement>>,
    outputs: Vec<Rc<GenericElement>>,
    is_locked: bool,

    // This is used specifically for structural problems where the original position of the node must be known.
    //_metadata: Option<Matrix<f64>>, 
}
impl GenericNode
{
    pub fn new() -> Rc<RefCell<GenericNode>>
    {
        Rc::new(RefCell::new(GenericNode
        {
            potential: col_vec![1f64],
            inputs: vec![],
            outputs: vec![],
            is_locked: false,
            //_metadata: None,
        }))
    }

    pub fn get_flux_discrepancy(&self) -> anyhow::Result<Matrix<f64>>
    {
        let mut ret_val = Matrix::new(
            self.potential.get_rows(),
            self.potential.get_cols(),
        ); 

        for elem in &self.inputs
        {
            ret_val += elem.get_flux()?;
        }

        for elem in &self.outputs
        {
            ret_val -= elem.get_flux()?;
        }

        return Ok(ret_val)
    } 
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct ComponentIndex
{
    node: u32,
    component: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NodalAnalysisStudyResult
{
    nodes: HashMap<u32, Vec<f64>>,
    elements: HashMap<u32, Vec<f64>>,
}

#[derive(Clone, Debug)] 
pub struct NodalAnalysisStudy<T>
{
    elements: Vec<Rc<GenericElement>>,
    nodes: Vec<Rc<RefCell<GenericNode>>>,
    _phantom: PhantomData<T>,
}
impl <T> NodalAnalysisStudy<T>
{
    pub fn new() -> NodalAnalysisStudy<T>
    {
        NodalAnalysisStudy
        {
            elements: vec![],
            nodes: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn add_nodes(&mut self, n: usize)
    {
        for _ in 0..n
        {
            self.nodes.push(GenericNode::new());
        }
    }

    pub fn add_element(&mut self, element_type: ElementConstructor<T>, input: usize, output: usize, value: T) -> anyhow::Result<()>
    {
        let elem = (element_type)(
            Rc::downgrade(&self.nodes[input]), 
            Rc::downgrade(&self.nodes[output]), 
            value)?;

        self.elements.push(elem);

        // println!("Connected node {input} to node {output} with {element_type:#?}.");
        Ok(())
    }

    pub fn ground_node(&mut self, node: usize)
    {
        // println!("Grounded node: {node}");

        let mut grounded_node = self.nodes[node].borrow_mut();
        let n = grounded_node.potential.get_rows();
        
        for i in 0..n
        {
            grounded_node.potential[(i, 0)] = 0.0;
        }

        grounded_node.is_locked = true;
    }

    fn generate_system(&self) -> anyhow::Result<(
        Vec<impl Fn(&HashMap<ComponentIndex, f64>) -> anyhow::Result<f64>>,
        HashMap<ComponentIndex, f64>, 
    )>
    {
        let num_components = match self.nodes.first()
        {
            Some(node) => node.try_borrow()?.potential.get_rows(),
            None => return Err(EquationGenerationError::NoNodesInSystem.into()),
        };

        if self.nodes.len() > u32::MAX as usize ||
           num_components > u32::MAX as usize
        {
            return Err(EquationGenerationError::NodeCountIntegerOverflow.into())
        }

        let mut independents = HashMap::new();
        let mut dependents = Vec::new();
        
        for (i, node) in (&self.nodes).iter()
            .enumerate()
            .filter(|x| !(x.1.borrow().is_locked)) // this is ok. the borrow will be dropped when the closure returns
        {
            // println!("sigma balls {i}");

            for (j, component) in node.try_borrow()?
                .potential
                .iter()
                .enumerate()
            {
                // Get the position of this component in the jacobian
                let idx = ComponentIndex 
                { 
                    node: i as u32, 
                    component: j as u32 
                };

                independents.insert(idx, *component);
                
                let local_node = Rc::clone(&self.nodes[i]);

                dependents.push(move |x: &HashMap<ComponentIndex, f64>| {
                    let p_init;
                    let flux_discrepancy;
                    {
                        // println!("Getting the initial value and overwriting the nodal potential...");
                        p_init = local_node.try_borrow()?.potential[(j, 0)];
                        local_node.try_borrow_mut()?.potential[(j, 0)] = x[&idx];
                        // println!("Done!");
                    }
                    {
                        // println!("Getting the value of interest and setting the value back to initial state...");
                        flux_discrepancy = local_node.try_borrow()?.get_flux_discrepancy()?[(j, 0)];
                        local_node.try_borrow_mut()?.potential[(j, 0)] = p_init;
                        // println!("Done!");
                    }
                    Ok(flux_discrepancy)
                });
            }
        }
        Ok((dependents, independents))
    }

    pub fn solve(&self) -> anyhow::Result<NodalAnalysisStudyResult>
    {
        let mut ret_val = NodalAnalysisStudyResult 
        { 
            nodes: HashMap::new(), 
            elements: HashMap::new(),
        };
        let (f, mut guess) = self.generate_system()?;
        let soln = multivariate_newton_raphson(f, &mut guess, 0.0001, 1000)?;

        // Set nodal potentials to solution
        for (idx, component) in soln
        {
            let mut node = self.nodes[idx.node as usize].try_borrow_mut()?;
            node.potential[(idx.component as usize, 0)] = *component;
        }

        // Get all elemental flux values for solution 
        // (do elements first so non-dof nodes have correct potential set)
        for (idx, elem) in self.elements.iter().enumerate()
        {
            ret_val.elements.insert(
                idx as u32, 
                elem.get_flux()?.into(),
            );
        }

        // Get all nodal potential values for solution
        for (idx, node) in self.nodes.iter().enumerate()
        {
            ret_val.nodes.insert(
                idx as u32, 
                node.try_borrow()?.potential.clone().into(),
            );
        }

        return Ok(ret_val)
    }
}

/// Returns a boolean indicating whether the `GenericNode` at the given pointer 
/// is locked or not. This function will return a `DroppedNodeError` if the 
/// node was dropped for some reason prior to checking the state of `is_locked`.
/// 
/// # Example
/// ```
/// use std::rc::Rc;
/// use neapolitan::{GenericNode, is_locked};
/// 
/// let my_node_ref = GenericNode::new();
/// 
/// assert!(
///     !(is_locked(&Rc::downgrade(&my_node_ref)).unwrap())
/// )
/// ```
pub fn is_locked(node_ref: &Weak<RefCell<GenericNode>>) -> anyhow::Result<bool>
{
    if let Some(node) = node_ref.upgrade()
    {
        Ok(node.try_borrow()?.is_locked)
    }
    else 
    {
        Err(DroppedNodeError.into())
    }
}

pub fn lock_node(node_ref: &Weak<RefCell<GenericNode>>) -> anyhow::Result<()>
{
    if let Some(node) = node_ref.upgrade()
    {
        node.try_borrow_mut()?.is_locked = true;
        Ok(())
    }
    else 
    {
        Err(DroppedNodeError.into())
    }
}