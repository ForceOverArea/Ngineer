use std::rc::Rc;
use std::cell::RefCell;

use gmatlib::{col_vec, Matrix};
use crate::errors::ElementCreationError;
use crate::{GenericElement, GenericNode};
use crate::flux_formulas::*;

pub fn resistor(input: Rc<RefCell<GenericNode>>, output: Rc<RefCell<GenericNode>>, resistance: f64) -> anyhow::Result<GenericElement>
{
    Ok(GenericElement
    {
        gain: col_vec![1f64 / resistance], // Convert resistance to conductance
        input_node: input,
        output_node: output,
        flux_calc: normal_flux,
        drives_output: false,
    })
}

pub fn voltage_source(input: Rc<RefCell<GenericNode>>, output: Rc<RefCell<GenericNode>>, voltage: f64) -> anyhow::Result<GenericElement>
{
    // Abort if we cannot remove a DOF from the problem
    if output.borrow().is_locked &&
       input.borrow().is_locked
    {
        return Err(ElementCreationError.into());
    }

    let drives_output = !output.borrow().is_locked;
    Ok(GenericElement
    {
        gain: col_vec![voltage], // Convert resistance to conductance
        input_node: input,
        output_node: output,
        flux_calc: observe_flux,
        drives_output,
    })
}

pub fn current_source(input: Rc<RefCell<GenericNode>>, output: Rc<RefCell<GenericNode>>, current: f64) -> anyhow::Result<GenericElement>
{
    Ok(GenericElement
    {
        gain: col_vec![current], // Convert resistance to conductance
        input_node: input,
        output_node: output,
        flux_calc: constant_flux,
        drives_output: false,
    })
}