
pub mod modelling;
/// Contains error types for the various errors that 
/// may be thrown in the `neapolitan` crate.
pub mod errors;
/// Contains functions used by the nodal analysis 
/// elements to calculate elemental flux between nodes.
pub mod flux_formulas;

/// Contains constructor functions for elements useful in
/// modelling steady-state DC circuits.
pub mod ssdc_circuits;

// Standard modules
use std::collections::HashMap;
use std::{fmt::Debug, marker::PhantomData};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use modelling::{element, NodalAnalysisModel};
// 3rd party modules
use serde::{Deserialize, Serialize};
use geqslib::newton::multivariate_newton_raphson;

/// This is a re-export of a `gmatlib::Matrix<T>`, a type for representing numerical 
/// matrices and vectors and operating on them in a more math-oriented way.
/// 
/// `Matrix<T>`s represent a single, contiguous piece of memory that can be used in 
/// source code as if it were an m x n matrix quantity. It implements multiplication,
/// addition, and subtraction operations, and can also be inverted, sliced and 
/// mutably/immutably indexed.
/// 
/// # For more info
/// please see the [gmatlib docs](https://docs.rs/gmatlib/0.2.0/gmatlib/).
pub type Matrix<T> = gmatlib::Matrix<T>;

// Local modules
use errors::{DroppedNodeError, EquationGenerationError};
use modelling::element::{ElementConstructor, GenericElement};
use modelling::node::GenericNode;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
struct ComponentIndex
{
    node: u32,
    component: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NodalAnalysisStudyResult
{
    nodes: HashMap<u32, Vec<f64>>,
    elements: HashMap<u32, Vec<f64>>,
}

/// A builder struct for building a customized instance of 
/// the Neapolitan nodal analysis solver engine. This allows a
/// user to extend it's functionality by adding custom elements
/// and flux calculations to the engine's vocabulary.
/// 
/// # Example
/// ```
/// 
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct NodalAnalysisStudyConfigurator
{
    parent: &NodalAnalysisStudyBuilder,
    dimension: usize,
    elements: HashMap<String, ElementConstructor>,
}
impl NodalAnalysisStudyConfigurator
{
    /// Creates a new `NodalAnalysisStudyConfigurator` instance, allowing 
    /// a user to create a customized instance of the Neapolitan solver engine.
    /// 
    /// # Example
    /// ```
    /// 
    /// ```
    pub fn new(dimension: usize) -> NodalAnalysisStudyConfigurator
    {
        NodalAnalysisStudyConfigurator
        {
            dimension,
            elements: HashMap::new(),
        }
    }

    /// Adds a custom element to the study configuration, allowing a user to 
    /// extend the variety of available elements in the solver engine.
    /// 
    /// # Example
    /// ```
    /// 
    /// ``` 
    pub fn add_element_type(mut self, name: &str, element_type: ElementConstructor) -> NodalAnalysisStudyConfigurator
    {
        self.elements.insert(
            name.to_string(), 
            element_type
        );

        self
    }

    pub fn configure(self) -> NodalAnalysisStudyBuilder
    {

    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodalAnalysisStudyBuilder
{
    configurator: HashMap<String, NodalAnalysisStudyConfigurator>,
    model: NodalAnalysisModel, 
}
impl NodalAnalysisStudyBuilder
{
    pub fn new(study_type: &str) -> anyhow::Result<NodalAnalysisStudyBuilder>
    {
        Ok(
            NodalAnalysisStudyBuilder 
            {
                configurator: HashMap::new(),
                model: NodalAnalysisModel { 
                    model_type: 
                    study_type.to_string(), 
                    nodes: 0, 
                    configuration: HashMap::new(),
                    elements: vec![],
                },
            }
        )
    }
}

/// Returns a boolean indicating whether the `GenericNode` at the given pointer 
/// is locked or not. This function will return a `DroppedNodeError` if the 
/// node was dropped for some reason prior to checking the state of `is_locked`.
/// 
/// # Example
/// ```
/// use std::rc::Rc;
/// use neapolitan::is_locked;
/// use neapolitan::modelling::GenericNode;
/// 
/// let my_node_ref = GenericNode::new();
/// 
/// assert!(
///     !(is_locked(&Rc::downgrade(&my_node_ref)).unwrap())
/// )
/// ```
pub fn is_locked(node_ref: &Weak<RefCell<GenericNode>>) -> anyhow::Result<bool>
{
    if let Some(node) = node_ref.upgrade()
    {
        Ok(node.try_borrow()?.is_locked)
    }
    else 
    {
        Err(DroppedNodeError.into())
    }
}

pub fn lock_node(node_ref: &Weak<RefCell<GenericNode>>) -> anyhow::Result<()>
{
    if let Some(node) = node_ref.upgrade()
    {
        node.try_borrow_mut()?.is_locked = true;
        Ok(())
    }
    else 
    {
        Err(DroppedNodeError.into())
    }
}

pub fn get_node_potential(node_ref: &Weak<RefCell<GenericNode>>) -> anyhow::Result<Matrix<f64>>
{
    if let Some(node) = node_ref.upgrade()
    {
        Ok(node.try_borrow_mut()?.potential.clone())
    }
    else
    {
        Err(DroppedNodeError.into())
    }
}

pub fn set_node_potential(node_ref: &Weak<RefCell<GenericNode>>, potential: Vec<f64>) -> anyhow::Result<()>
{
    if let Some(node) = node_ref.upgrade()
    {
        node.try_borrow_mut()?.potential = Matrix::from_col_vec(potential);
        Ok(())
    }
    else
    {
        Err(DroppedNodeError.into())
    }
}
