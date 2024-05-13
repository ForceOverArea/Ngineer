/// Contains functions used by the nodal analysis 
/// elements to calculate elemental flux between nodes.
pub mod flux_formulas;

pub mod ss1d_circuits;

pub mod errors;

use std::collections::HashMap;
// Standard modules
use std::{fmt::Debug, marker::PhantomData};
use std::rc::Rc;
use std::cell::RefCell;

// Local modules
use flux_formulas::*;
use gmatlib::{col_vec, Matrix};
use geqslib::newton::multivariate_newton_raphson;

pub type ElementConstructor<T> = fn (Rc<RefCell<GenericNode>>, Rc<RefCell<GenericNode>>, T) -> anyhow::Result<GenericElement>;

#[derive(Clone, Debug, PartialEq)] //, PartialOrd)]
pub struct GenericElement
{
    gain: Matrix<f64>,
    input_node: Rc<RefCell<GenericNode>>,
    output_node: Rc<RefCell<GenericNode>>,
    flux_calc: FluxCalculation,
    drives_output: bool,
}

#[derive(Clone, Debug, PartialEq)] //, PartialOrd)]
pub struct GenericNode
{
    potential: Matrix<f64>,
    inputs: Vec<Rc<GenericElement>>,
    outputs: Vec<Rc<GenericElement>>,
    is_locked: bool,

    // This is used specifically for structural problems where the original position of the node must be known.
    metadata: Option<Matrix<f64>>, 
}
impl GenericNode
{
    pub fn new_ref() -> Rc<RefCell<GenericNode>>
    {
        Rc::new(RefCell::new(GenericNode
        {
            potential: col_vec![1f64],
            inputs: vec![],
            outputs: vec![],
            is_locked: false,
            metadata: None,
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
            let flux = (elem.flux_calc)(
                Rc::clone(&elem.input_node), Rc::clone(&elem.output_node), 
                &elem.gain, elem.drives_output)?;

            ret_val += flux;
        }

        for elem in &self.outputs
        {
            let flux = (elem.flux_calc)(
                Rc::clone(&elem.input_node), Rc::clone(&elem.output_node), 
                &elem.gain, elem.drives_output)?;

            ret_val -= flux;
        }

        return Ok(ret_val)
    } 
}

#[derive(Clone, Debug, PartialEq)] 
pub struct NodalAnalysisStudy<T>
{
    elements: Vec<GenericElement>,
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
            self.nodes.push(GenericNode::new_ref());
        }
    }

    pub fn add_element(&mut self, element_type: ElementConstructor<T>, input: usize, output: usize, value: T) -> anyhow::Result<()>
    {
        let elem = (element_type)(
            Rc::clone(&self.nodes[input]), 
            Rc::clone(&self.nodes[output]), 
            value)?;
        
        self.elements.push(elem);

        Ok(())
    }

    fn generate_system(&self) -> anyhow::Result<(
        HashMap<String, f64>, 
        Vec<impl Fn(&HashMap<String, f64>) -> anyhow::Result<f64>>
    )>
    {
        let mut independents = HashMap::new();
        let mut dependents = Vec::new();
        
        for (i, node) in (&self.nodes).iter().enumerate()
        {
            for (j, component) in node.try_borrow()?.potential.iter().enumerate()
            {
                independents.insert(format!("{i},{j}"), *component);
                
                let local_node_ref = Rc::clone(&self.nodes[i]);

                dependents.push(move |x: &HashMap<String, f64>| {
                    // Get access to the node
                    let mut local_node = local_node_ref.try_borrow_mut()?;

                    let p_init = local_node.potential[(j, 0)];                  // Get the initial value
                    local_node.potential[(j, 0)] = x[&format!("{i},{j}")];      // Overwrite the nodal potential
                    let ret_val = local_node.get_flux_discrepancy()?[(j, 0)];   // Get the value of interest
                    local_node.potential[(j, 0)] = p_init;                      // Set the value back to initial state

                    Ok(ret_val)
                });
            }
        }
        Ok((independents, dependents))
    }

    pub fn solve(&self) -> anyhow::Result<()>
    {
        let (guess, f) = self.generate_system()?;
        let soln = multivariate_newton_raphson(f, &mut guess, 0.0001, 1000)?;

    }
}