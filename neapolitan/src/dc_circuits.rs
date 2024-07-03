// Std modules
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 3rd party modules
use gmatlib::{col_vec, Matrix};

// Local modules
use crate::errors::ElementCreationError;
use crate::{flux_formulas::*, get_node_potential, is_locked, lock_node, set_node_potential};
use crate::{GenericElement, GenericNode};

pub const DC_CIRCUIT: &str = "dc_circuit";
pub const RESISTOR: &str = "resistor";
pub const VOLTAGE_SOURCE: &str = "voltage_source";
pub const CURRENT_SOURCE: &str = "current_source";

pub fn resistor(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    resistance: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    GenericElement::try_new(
        vec![1.0 / resistance[0]],  // Conductance (gain) is reciprocal of resistance in ohms
        input_node, output_node,    // Input and output_node nodes
        normal_flux,                // Flux calculation
        false,                      // Does not drive a nodal potential
        true, true,                 // Connect to both nodes
    )
}

pub fn voltage_source(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    voltage: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    // Abort if we cannot remove a DOF from the problem
    if is_locked(&output_node)? && is_locked(&input_node)?
    {
        return Err(ElementCreationError.into())
    }

    // Determine if we're driving the input or output node
    let drives_output = !is_locked(&output_node)?;

    // Remove the appropriate DOF
    if drives_output
    {
        lock_node(&output_node)?;
        set_node_potential(&output_node, (get_node_potential(&input_node)? + col_vec![voltage[0]]).into())?;
    }
    else // driving input_node node:
    {
        lock_node(&input_node)?;
        set_node_potential(&input_node, (get_node_potential(&output_node)? + col_vec![voltage[0]]).into())?;
    }

    // If we're driving the output node, we need to make the input node aware of this element.
    let connect_input_node = drives_output;

    // If we're not going to make the input aware of this element, make the output node aware.
    let connect_output_node = !connect_input_node;
    
    GenericElement::try_new(
        voltage,
        input_node, output_node,
        observe_flux,
        drives_output,
        connect_input_node,
        connect_output_node,
    )
}

pub fn current_source(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    current: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    GenericElement::try_new(
        current,
        input_node, output_node,
        constant_flux,
        false,
        true, true,
    )
}
