use gmatlib::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

use crate::GenericNode;

/// The function signature used to calculate flux between nodes.
pub type FluxCalculation = fn (Rc<RefCell<GenericNode>>, Rc<RefCell<GenericNode>>, &Matrix<f64>, bool) -> anyhow::Result<Matrix<f64>>;

pub (in crate) fn normal_flux(
    inode_ref: Rc<RefCell<GenericNode>>, 
    onode_ref: Rc<RefCell<GenericNode>>, 
    gain: &Matrix<f64>, 
    _drives_output: bool
) -> anyhow::Result<Matrix<f64>>
{
    let onode = onode_ref.try_borrow()?;
    let inode = inode_ref.try_borrow()?;

    let deltas = &(onode.potential) - &(inode.potential);

    Ok(deltas * gain)
}

pub (in crate) fn observe_flux(
    inode_ref: Rc<RefCell<GenericNode>>, 
    onode_ref: Rc<RefCell<GenericNode>>, 
    delta: &Matrix<f64>, 
    drives_output: bool
) -> anyhow::Result<Matrix<f64>>
{
    // Select node to set potential of
    let (sub_ref, dom_ref) = match drives_output
    {
        true  => (onode_ref, inode_ref),
        false => (inode_ref, onode_ref),
    };

    let mut sub = sub_ref.try_borrow_mut()?;
    let dom = dom_ref.try_borrow()?;

    // Adjust potential of submissive node
    sub.potential = &(dom.potential) + delta;

    Ok(sub.get_flux_discrepancy()?)
}

pub (in crate) fn constant_flux(
    _inode_ref: Rc<RefCell<GenericNode>>, 
    _onode_ref: Rc<RefCell<GenericNode>>, 
    flux: &Matrix<f64>, 
    _drives_output: bool
) -> anyhow::Result<Matrix<f64>>
{
    Ok(flux.clone())
}