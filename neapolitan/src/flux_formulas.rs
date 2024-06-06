use gmatlib::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

use crate::modelling::node::GenericNode;

pub fn normal_flux(
    inode_ref: Rc<RefCell<GenericNode>>, 
    onode_ref: Rc<RefCell<GenericNode>>, 
    gain: &Matrix<f64>, 
    _drives_output: bool
) -> anyhow::Result<Matrix<f64>>
{
    let onode = onode_ref.try_borrow()?;
    let inode = inode_ref.try_borrow()?;

    let mut deltas = &(inode.potential) - &(onode.potential);
    deltas.inplace_scale(gain[(0, 0)]);
    Ok(deltas)
}

pub fn observe_flux(
    inode_ref: Rc<RefCell<GenericNode>>, 
    onode_ref: Rc<RefCell<GenericNode>>, 
    delta: &Matrix<f64>, 
    drives_output: bool
) -> anyhow::Result<Matrix<f64>>
{
    let sub_ref;

    // Adjust potential of submissive node and drop mutable ref
    if drives_output
    {
        let mut sub = onode_ref.try_borrow_mut()?;
        let dom = inode_ref.try_borrow()?;

        // Add delta if output node is the driver
        sub.potential = &(dom.potential) + delta;
        drop(sub);

        sub_ref = onode_ref;
    }
    else
    {
        let mut sub = inode_ref.try_borrow_mut()?;
        let dom = onode_ref.try_borrow()?;

        // Subtract delta if output node is the driver
        sub.potential = &(dom.potential) - delta;
        drop(sub);

        sub_ref = inode_ref;
    }

    let mut discrepancy = sub_ref.try_borrow()?
        .get_flux_discrepancy()?;

    discrepancy.inplace_scale(-1.0);

    Ok(discrepancy)
}

pub fn constant_flux(
    _inode_ref: Rc<RefCell<GenericNode>>, 
    _onode_ref: Rc<RefCell<GenericNode>>, 
    flux: &Matrix<f64>, 
    _drives_output: bool
) -> anyhow::Result<Matrix<f64>>
{
    Ok(flux.clone())
}