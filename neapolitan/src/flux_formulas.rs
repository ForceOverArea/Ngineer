use std::cell::RefCell;
use std::rc::Rc;
use gmatlib::Matrix;
use crate::GenericNode;

/// The function signature used to calculate flux between nodes.
pub type FluxCalculation = fn (Rc<RefCell<GenericNode>>, Rc<RefCell<GenericNode>>, &Matrix<f64>, bool) -> Option<Matrix<f64>>;

fn normal_flux(
    inode_ref: Rc<RefCell<GenericNode>>, 
    onode_ref: Rc<RefCell<GenericNode>>, 
    gain: &Matrix<f64>, 
    _phantom_arg: bool
) -> Option<Matrix<f64>>
{
    if let (Ok(onode), Ok(inode)) = (onode_ref.try_borrow(), inode_ref.try_borrow())
    {
        let deltas = &(onode.potential) - &(inode.potential);
        return Some(deltas * gain)
    }
    None
}

fn observe_flux(
    inode_ref: Rc<RefCell<GenericNode>>, 
    onode_ref: Rc<RefCell<GenericNode>>, 
    delta: &Matrix<f64>, 
    drives_output: bool
) -> Option<Matrix<f64>>
{
    // Select node to set potential of
    let (sub_ref, dom_ref) = match drives_output
    {
        true => (onode_ref, inode_ref),
        false => (inode_ref, onode_ref),
    };

    // Adjust potential of submissive node
    if let (Ok(mut sub), Ok(dom)) = (sub_ref.try_borrow_mut(), dom_ref.try_borrow())
    {
        sub.potential = &(dom.potential) + delta;
        
        return Some(sub.get_flux_discrepancy())
    }
        
    None
}

fn constant_flux(
    _inode_ref: Rc<RefCell<GenericNode>>, 
    _onode_ref: Rc<RefCell<GenericNode>>, 
    delta: &Matrix<f64>, 
    _drives_output: bool
) -> Option<Matrix<f64>>
{
    Some(delta.clone())
}