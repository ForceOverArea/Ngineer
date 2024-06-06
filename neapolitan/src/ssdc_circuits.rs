// Std modules
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 3rd party modules
use gmatlib::{col_vec, Matrix};

// Local modules
use crate::errors::ElementCreationError;
use crate::{flux_formulas::*, get_node_potential, is_locked, lock_node, set_node_potential, Configure};
use crate::{GenericElement, GenericNode, NodalAnalysisStudy};

pub type SSDCCircuit = NodalAnalysisStudy<f64, Configure>;

pub fn resistor(
    input: Weak<RefCell<GenericNode>>, 
    output: Weak<RefCell<GenericNode>>, 
    resistance: f64
) -> anyhow::Result<Rc<GenericElement>>
{
    GenericElement::try_new(
        vec![1.0 / resistance], // Conductance (gain) is reciprocal of resistance in ohms
        input, output,              // Input and output nodes
        normal_flux,                // Flux calculation
        false,                      // Does not drive a nodal potential
        true, true,                 // Connect to both nodes
    )
}

pub fn voltage_source(
    input: Weak<RefCell<GenericNode>>, 
    output: Weak<RefCell<GenericNode>>, 
    voltage: f64
) -> anyhow::Result<Rc<GenericElement>>
{
    // Abort if we cannot remove a DOF from the problem
    if is_locked(&output)? && is_locked(&input)?
    {
        return Err(ElementCreationError.into())
    }

    // Determine if we're driving the input or output node
    let drives_output = !is_locked(&output)?;

    // Remove the appropriate DOF
    if drives_output
    {
        lock_node(&output)?;
        set_node_potential(&output, (get_node_potential(&input)? + col_vec![voltage]).into())?;
    } 
    else // driving input node:
    {
        lock_node(&input)?;
        set_node_potential(&input, (get_node_potential(&output)? + col_vec![voltage]).into())?;
    }

    // If we're driving the output node, we need to make the input node aware of this element.
    let connect_input_node = drives_output;

    // If we're not going to make the input aware of this element, make the output node aware.
    let connect_output_node = !connect_input_node;
    
    GenericElement::try_new(
        vec![voltage],
        input, output,
        observe_flux,
        drives_output,
        connect_input_node,
        connect_output_node,
    )
}

pub fn current_source(
    input: Weak<RefCell<GenericNode>>, 
    output: Weak<RefCell<GenericNode>>, 
    current: f64
) -> anyhow::Result<Rc<GenericElement>>
{
    GenericElement::try_new(
        vec![current],
        input, output,
        constant_flux,
        false,
        true, true,
    )
}