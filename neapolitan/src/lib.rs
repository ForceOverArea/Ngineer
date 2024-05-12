/// Contains functions used by the nodal analysis 
/// elements to calculate elemental flux between nodes.
pub mod flux_formulas;

// Standard modules
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;

// Local modules
use flux_formulas::*;
use gmatlib::Matrix;

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
}
impl GenericNode
{
    pub fn get_flux_discrepancy(&self) -> Matrix<f64>
    {
        let mut ret_val = Matrix::new(
            self.potential.get_rows(),
            self.potential.get_cols(),
        ); 

        for elem in self.inputs.iter()
            .map(|x| Rc::clone(&x))
        {
            if let Some(flux) = (elem.flux_calc)(
                Rc::clone(&elem.input_node), 
                Rc::clone(&elem.output_node), // Can this be improved?
                &elem.gain, 
                elem.drives_output)
            {
                ret_val = ret_val + flux;
            }
            else
            {

            }
        }

        ret_val
    } 
}