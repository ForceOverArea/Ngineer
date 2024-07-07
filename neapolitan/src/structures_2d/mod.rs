mod cross_sections;

// Std modules
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// 3rd party modules
use gmatlib::{col_vec, Matrix};
use anyhow;

// Local modules
pub use cross_sections::*;
use crate::errors::ElementCreationError;
use crate::{flux_formulas::*, get_node_potential, is_locked, lock_node, set_node_potential};
use crate::{GenericElement, GenericNode};

pub const STRUCTURES_2D: &str = "2d_structures"; 
pub const BEAM_2D: &str = "2d_beam";

pub fn square_beam(
    input_node: Weak<RefCell<GenericNode>>, 
    output_node: Weak<RefCell<GenericNode>>, 
    width_height: Vec<f64>,
) -> anyhow::Result<Rc<GenericElement>>
{
    if width_height.len() != 2
    {
        // return Err(ConvectionInterFaceCreationError.into());
    }

    let cs = Square { width: width_height[0], height: width_height[1] };

    GenericElement::try_new(
        vec![cs.ix(), cs.area(), ], 
        input_node, output_node, 
        , 
        false, 
        true, true
    )
}