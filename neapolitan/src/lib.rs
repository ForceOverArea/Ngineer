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
use gmatlib::{col_vec, Matrix};
use geqslib::newton::multivariate_newton_raphson;

// Local modules
use errors::{DroppedNodeError, EquationGenerationError, FluxCalculationError};

/// The function signature for a `neapolitan`-compatible element constructor function.
/// 
/// # Design Philosophy:
/// Because Rust does not support inheritance of shared data between types, 
/// extending `neapolitan` requires that authors write a "constructor function" that 
/// packs the relevant data into a `GenericElement` for use by the solver engine. This 
/// design pattern allows users to create a table in source describing how to build
/// their model from nodes and elements.
/// 
/// # Argument Types:
/// The type `T` is variable to allow authors to format data used in deriving the `gain`
/// value as well as additional metadata used in flux calculations. For example, in 
/// steady-state circuitry, `T` is simply `f64` to represent a real value for the voltage,
/// resistance, or current of the element being created. In AC circuitry problems,
/// however, this type is `(f64, f64)` to represent the real and imaginary components of
/// current at that location.
/// 
/// Constructor functions operate on `Weak<RefCell<GenericNode>>` smart pointers in order to
/// properly create the network structure used to model problems. These node references must
/// be passed on to the created element, but may be operated on prior to element creation. 
/// E.g. `voltage_source` sets a driven node's voltage to the driving node's potential +/- the
/// potential difference specified prior to connecting the element to the network. 
pub type ElementConstructor<T> = fn (Weak<RefCell<GenericNode>>, Weak<RefCell<GenericNode>>, T) -> anyhow::Result<Rc<GenericElement>>;

/// The function signature used to calculate flux between nodes.
/// 
/// # Design Philosophy
/// Flux calculation functions are the other function that must be defined in order to extend 
/// `neapolitan`. They are implemented as a single function pointer rather than a trait object
/// for simplicity. This ensures that implementors do not have to define a type that does not 
/// cleanly mimic a `GenericElement` just to implement a common method.
/// 
/// These functions get called by the `GenericElement::get_flux` method. They must 
/// return a `Matrix<f64>` containing the flux value that the element should have given its 
/// nodal potentials and `gain` value. Similarly to `ElementConstructor<T>`, this function type 
/// must work with smart pointers to `GenericNode`s, but this time must work with `Rc<RefCell<GenericNode>>` 
/// instead of `Weak<RefCell<GenericNode>>` as the type. They additionally take a `&Matrix<f64>` 
/// argument that will be equivalent to the element's `gain` value and a `bool` that indicates 
/// the directionality of some quantity. This is arbitrary and up to the implementor to choose 
/// its meaning.
/// 
/// # Note on Runtime Borrowing:
/// Upon being called, these functions may have more or less whatever side effects they need to in
/// order to make their calculations work. One key thing to keep in mind however, is that elements
/// that require a node's flux discrepancy value in order to determine a flux must **not**
/// mutably borrow that node or any other node used in the flux discrepancy calculation to avoid
/// raising an `AlreadyBorrowed` error.  
pub type FluxCalculation = fn (Rc<RefCell<GenericNode>>, Rc<RefCell<GenericNode>>, &Matrix<f64>, bool) -> anyhow::Result<Matrix<f64>>;

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
    /// Attempts to construct a new `GenericElement`, possibly returning a `DroppedNodeError`
    /// if the nodes being connected were previously dropped for some reason. This method is
    /// intended for use in an `ElementConstructor<T>`-compatible function.
    /// 
    /// # Arguments of Interest
    /// `flux_calc` - a function pointer to the flux calculation that this element should perform.
    /// 
    /// `drives_output` - an arbitrary value used to indicate directionality. For example, voltage
    /// source elements in DC circuitry problems use this to determine whether they should control
    /// the input or output node's potential value.
    /// 
    /// `connect_to_input` and `connect_to_output` - boolean flags that indicate whether the input 
    /// and output nodes, respectively, should be made aware of the element. Usually these will 
    /// always be true, but elements that use an adjacent node's flux balance in their flux 
    /// calculation may need to intentionally not connect to that element. If they do, they will 
    /// recursively call `GenericElement::get_flux` and cause a stack overflow unless some other 
    /// action is taken.
    /// 
    /// # Example
    /// ```
    /// use std::rc::{Rc, Weak};
    /// use std::cell::RefCell;
    /// use neapolitan::{GenericElement, GenericNode};
    /// use neapolitan::flux_formulas::constant_flux;
    /// use neapolitan::errors::DroppedNodeError;
    /// 
    /// pub fn current_source(
    ///     input: Weak<RefCell<GenericNode>>, 
    ///     output: Weak<RefCell<GenericNode>>, 
    ///     current: f64
    /// ) -> Result<Rc<GenericElement>, Box<dyn std::error::Error>>
    /// {
    ///     Ok(GenericElement::try_new(
    ///         vec![current],
    ///         input, output,
    ///         constant_flux,
    ///         false,      // We don't need this information. Just make it `false` 
    ///         true, true, // Connect the input and output to the element.
    ///     )?)
    /// }
    /// ```
    pub fn try_new(gain: Vec<f64>,
        input_node: Weak<RefCell<GenericNode>>,
        output_node: Weak<RefCell<GenericNode>>,
        flux_calc: FluxCalculation,
        drives_output: bool,
        connect_to_input: bool,
        connect_to_output: bool,
    ) -> anyhow::Result<Rc<GenericElement>>
    {
        let elem = Rc::new(
            GenericElement 
            {
                gain: Matrix::from_col_vec(gain), 
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
                input_refcell.try_borrow_mut()?
                    .outputs.push(Rc::clone(&elem));
            }

            if connect_to_output
            {
                output_refcell.try_borrow_mut()?
                    .inputs.push(Rc::clone(&elem));
            }

            Ok(elem)
        }
        else 
        {
            Err(DroppedNodeError.into())
        }
    }

    /// Calculates the flux for this element by calling the `FluxCalculation` function pointer
    /// it owns.
    /// 
    /// # Example
    /// ```
    /// use std::rc::Rc;
    /// use neapolitan::{GenericElement, GenericNode, set_node_potential};
    /// use neapolitan::ssdc_circuits::resistor;
    /// 
    /// let a = GenericNode::new();
    /// let b = GenericNode::new();
    /// 
    /// set_node_potential(&Rc::downgrade(&a), vec![3.0]).unwrap();
    /// 
    /// let elem = resistor(Rc::downgrade(&a), Rc::downgrade(&b), 2.0).unwrap();
    /// let flux = Vec::from(
    ///     elem.get_flux().unwrap()
    /// );
    /// 
    /// assert_eq!(vec![1.0], flux);
    /// ```
    pub fn get_flux(&self) -> anyhow::Result<Matrix<f64>>
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
    potential:  Matrix<f64>,
    inputs:     Vec<Rc<GenericElement>>,
    outputs:    Vec<Rc<GenericElement>>,
    is_locked:  bool,

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
        let mut inputs = Matrix::new(
            self.potential.get_rows(),
            self.potential.get_cols(),
        ); 

        let mut outputs = inputs.clone();

        for elem in &self.inputs
        {
            inputs += elem.get_flux()?;
        }

        for elem in &self.outputs
        {
            outputs += elem.get_flux()?;
        }

        println!("     Inputs = {inputs}");
        println!("    Outputs = {outputs}");

        let discrepancy = inputs - outputs;
        println!("Discrepancy = {discrepancy}");
        return Ok(discrepancy)
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

pub struct Configure;
pub struct Build;

#[derive(Clone, Debug)] 
pub struct NodalAnalysisStudy<T, S>
{
    elements: Vec<Rc<GenericElement>>,
    nodes: Vec<Rc<RefCell<GenericNode>>>,
    _phantom_type: PhantomData<T>,
    _phantom_state: PhantomData<S>,
}
impl <T, S> NodalAnalysisStudy<T, S>
{
    pub fn new() -> NodalAnalysisStudy<T, Configure>
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
        
        for (i, node) in (&self.nodes).iter()
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
                
                let local_nodes: Vec<Rc<RefCell<GenericNode>>> = self.nodes.iter()
                    .map(|x| x.clone())
                    .collect();

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