// Std modules
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 3rd party modules
use gmatlib::{col_vec, Matrix};
use thiserror::Error;

use crate::errors::ElementCreationError;
// Local modules
use crate::{flux_formulas::*, get_node_potential, is_locked, lock_node, set_node_potential};
use crate::{GenericElement, GenericNode};

#[derive(Debug, Error)]
#[error("you must specify a conductivity coefficient 'k' and a length (in that order), or calculate the ratio youself to create a conductor element")]
pub struct ConductorCreationError;

#[derive(Debug, Error)]
#[error("you must specify only a convection coefficient 'h' to create a convection_interface element")]
pub struct ConvectionInterFaceCreationError;

pub const HEAT_TRANSFER: &str = "heat_transfer";
pub const CONDUCTOR: &str = "conductor";
pub const CONVECTION_INTERFACE: &str = "convection_interface";
pub const TEMPERATURE_DELTA: &str = "temperature_delta";
pub const HEAT_FLUX: &str = "heat_flux";

/// Represents a simple 1-dimensional piece of conductive material with a
/// different temperature at each end and known thermal conductivity (often
/// denoted as 'K' in engineering courses or texts). 
/// 
/// This can be thought of as analogous to a `resistor` in DC circuits or an
/// `orifice` in hydraulic circuits, but the heat flux through a `conductor`
/// is directly proportional to the specified conductivity. The thermal conductivity
/// constant 'K' (measured in W/m-K or an equivalent unit) is a characteristic 
/// of the material that is conducting the heat in question in the given problem.
/// As a consequence of this, it is important to note that this element does not
/// consider the 
pub fn conductor(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    length_and_conductivity: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    let conductivity = match length_and_conductivity.len()
    {
        // If there is only 1 component to the vector, assume the user has calculated k / l in advance.
        1 => 
        { 
            length_and_conductivity
        },
        // If there are 2 components, assume we need to calculate the ratio ourselves.
        2 => 
        { 
            let l = length_and_conductivity[0];
            let k = length_and_conductivity[1];
            vec![k / l]
        },
        _ =>
        {
            return Err(ConductorCreationError.into());
        },
    };

    GenericElement::try_new(
        conductivity, 
        input_node, output_node,
        normal_flux, 
        false, 
        true, true
    )
}

pub fn convection_interface(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    convection_coef: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    if convection_coef.len() != 1
    {
        return Err(ConvectionInterFaceCreationError.into());
    }

    GenericElement::try_new(
        convection_coef, 
        input_node, output_node, 
        normal_flux, 
        false, 
        true, true
    )
}

pub fn temperature_delta(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    temp_delta: Vec<f64>,
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
        set_node_potential(&output_node, (get_node_potential(&input_node)? + col_vec![temp_delta[0]]).into())?;
    }
    else // driving input_node node:
    {
        lock_node(&input_node)?;
        set_node_potential(&input_node, (get_node_potential(&output_node)? + col_vec![temp_delta[0]]).into())?;
    }

    // If we're driving the output node, we need to make the input node aware of this element.
    let connect_input_node = drives_output;

    // If we're not going to make the input aware of this element, make the output node aware.
    let connect_output_node = !connect_input_node;
    
    GenericElement::try_new(
        temp_delta,
        input_node, output_node,
        observe_flux,
        drives_output,
        connect_input_node,
        connect_output_node,
    )
}

pub fn heat_flux(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    flux: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    GenericElement::try_new(
        flux, 
        input_node, output_node, 
        constant_flux, 
        false,
        true, true
    )
}