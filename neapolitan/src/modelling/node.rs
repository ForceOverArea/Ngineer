/// Std modules
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 3rd party modules
use gmatlib::col_vec;

/// Local modules 
use crate::Matrix;
use crate::modelling::element::GenericElement;

/// A struct representing a node in a nodal analysis problem.
/// 
/// # Concept:
/// Nodes are points where continuity/equilibrium must be enforced in a nodal 
/// analysis problem. For example, in circuitry problems, nodes are analogous 
/// to wires. In hydraulics problems, nodes are analogous to sections of hose 
/// not divided by a valve or other pressure-drop-inducing element. 
/// 
/// While not pertinent to *using* the neapolitan framework, *understanding* 
/// how nodes work is not very complicated. All `GenericElement` objects have
/// a `get_flux` method that returns a flux quantity. Based on whether elements
/// are connected to a node as an input or output, they add or subtract their flux 
/// quantities in a flux balance. This is similar to an "energy balance", "force
/// balance", "flow balance", etc. done in any number of engineering or physics 
/// courses, and enforces the continuity/equilibrium condition mentioned earlier at 
/// each node. The solver engine makes gradual changes to each nodes potential to
/// guide the nodal flux discrepancy of every node in the system to 0.
#[derive(Clone, Debug)]
pub struct GenericNode
{
    pub (in crate) potential: Matrix<f64>,
    pub (in crate) inputs: Vec<Rc<GenericElement>>,
    pub (in crate) outputs: Vec<Rc<GenericElement>>,
    pub (in crate) is_locked: bool,
    pub (in crate) _metadata: Option<HashMap<String, f64>>, 
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
            _metadata: None,
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

        let discrepancy = inputs - outputs;
        Ok(discrepancy)
    } 
}