use gmatlib::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

use crate::GenericNode;

pub fn normal_flux<M>(
    inode_ref: Rc<RefCell<GenericNode<M>>>, 
    onode_ref: Rc<RefCell<GenericNode<M>>>, 
    gain: &Matrix<f64>, 
    _drives_output: bool
) -> anyhow::Result<Matrix<f64>>
where M: Default
{
    let onode = onode_ref.try_borrow()?;
    let inode = inode_ref.try_borrow()?;

    let mut deltas = &(inode.potential) - &(onode.potential);
    deltas.inplace_scale(gain[(0, 0)]);
    Ok(deltas)
}

pub fn observe_flux<M>(
    inode_ref: Rc<RefCell<GenericNode<M>>>, 
    onode_ref: Rc<RefCell<GenericNode<M>>>, 
    delta: &Matrix<f64>, 
    drives_output: bool
) -> anyhow::Result<Matrix<f64>>
where M: Default
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

    let discrepancy = sub_ref.try_borrow()?.get_flux_discrepancy()?;
    Ok(discrepancy * -1.0)
}

pub fn constant_flux<M>(
    _inode_ref: Rc<RefCell<GenericNode<M>>>, 
    _onode_ref: Rc<RefCell<GenericNode<M>>>, 
    flux: &Matrix<f64>, 
    _drives_output: bool
) -> anyhow::Result<Matrix<f64>>
where M: Default
{
    Ok(flux.clone())
}