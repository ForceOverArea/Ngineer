
pub mod modelling;
/// Contains error types for the various errors that 
/// may be thrown in the `neapolitan` crate.
pub mod errors;
/// Contains functions used by the nodal analysis 
/// elements to calculate elemental flux between nodes.
pub mod flux_formulas;

/// Contains constructor functions for elements useful in
/// modelling steady-state DC circuits.
pub mod ssdc_circuits;

// Standard modules
use std::collections::HashMap;
use std::{fmt::Debug, marker::PhantomData};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 3rd party modules
use serde::Serialize;
use geqslib::newton::multivariate_newton_raphson;
pub use gmatlib::Matrix;

// Local modules
use errors::{DroppedNodeError, EquationGenerationError};
use modelling::element::{ElementConstructor, GenericElement};
use modelling::node::GenericNode;


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

pub struct Configure;
pub struct Build;

#[derive(Clone, Debug)] 
pub struct NodalAnalysisStudy<T, S = Configure>
{
    elements: Vec<Rc<GenericElement>>,
    nodes: Vec<Rc<RefCell<GenericNode>>>,
    _phantom_type: PhantomData<T>,
    _phantom_state: PhantomData<S>,
}
impl <T, S> Default for NodalAnalysisStudy<T, S>
{
    fn default() -> Self 
    {
        NodalAnalysisStudy
        {
            elements: vec![],
            nodes: vec![],
            _phantom_type: PhantomData,
            _phantom_state: PhantomData
        }
    }
}

impl <T> NodalAnalysisStudy<T, Configure>
{
    pub fn add_nodes(&mut self, n: usize)
    {
        for _ in 0..n
        {
            self.nodes.push(GenericNode::new());
        }
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

    pub fn configure(self) -> NodalAnalysisStudy<T, Build>
    {
        NodalAnalysisStudy
        {
            elements: self.elements,
            nodes: self.nodes,
            _phantom_type: PhantomData,
            _phantom_state: PhantomData
        }
    }
}

impl <T> NodalAnalysisStudy<T, Build>
{
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
        
        for (i, node) in self.nodes.iter()
            .enumerate()
            .filter(|x| !(x.1.borrow().is_locked)) // this is ok. the borrow will be dropped when the closure returns
        {
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
                
                let local_nodes: Vec<Rc<RefCell<GenericNode>>> = self.nodes.to_vec();

                dependents.push(move |x: &HashMap<ComponentIndex, f64>| {
                    println!("Node: {i}, Component: {j}\n  Potential = {} ", &x[&ComponentIndex{node: i as u32, component: j as u32}]);
                    
                    // Set values of all nodes
                    for (&ComponentIndex { node, component }, &val) in x
                    {
                        local_nodes[node as usize].try_borrow_mut()?
                            .potential[(component as usize, 0)] = val;
                    }

                    // Perform flux balance
                    let flux_discrepancy = local_nodes[i].try_borrow()?.get_flux_discrepancy()?;

                    // Report only component of interest
                    Ok(flux_discrepancy[(j, 0)])
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

        Ok(ret_val)
    }
}

/// Returns a boolean indicating whether the `GenericNode` at the given pointer 
/// is locked or not. This function will return a `DroppedNodeError` if the 
/// node was dropped for some reason prior to checking the state of `is_locked`.
/// 
/// # Example
/// ```
/// use std::rc::Rc;
/// use neapolitan::{GenericNode, is_locked, NoMetadata};
/// 
/// let my_node_ref = GenericNode::<NoMetadata>::new();
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

pub fn get_node_potential(node_ref: &Weak<RefCell<GenericNode>>) -> anyhow::Result<Matrix<f64>>
{
    if let Some(node) = node_ref.upgrade()
    {
        Ok(node.try_borrow_mut()?.potential.clone())
    }
    else
    {
        Err(DroppedNodeError.into())
    }
}

pub fn set_node_potential(node_ref: &Weak<RefCell<GenericNode>>, potential: Vec<f64>) -> anyhow::Result<()>
{
    if let Some(node) = node_ref.upgrade()
    {
        node.try_borrow_mut()?.potential = Matrix::from_col_vec(potential);
        Ok(())
    }
    else
    {
        Err(DroppedNodeError.into())
    }
}