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

use modelling::{element, NodalAnalysisElement, NodalAnalysisModel, NodalMetadata};
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
use errors::{DroppedNodeError, ElementCreationError, EquationGenerationError, NodalAnalysisConfigurationError, NodalAnalysisModellingError};
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
    parent: &'static NodalAnalysisStudyBuilder,
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
    pub fn new(dimension: usize, parent: &NodalAnalysisStudyBuilder) -> NodalAnalysisStudyConfigurator
    {
        NodalAnalysisStudyConfigurator
        {
            parent,
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
    pub fn add_element_type(mut self, name: &str, element_type: ElementConstructor) -> anyhow::Result<NodalAnalysisStudyConfigurator>
    {
        if let None = self.elements.insert(name.to_string(), element_type)
        {
            Ok(self)
        }
        else
        {
            Err(NodalAnalysisConfigurationError::ElementTypeNameCollision.into())
        }
    }

    /// Adds the information to the builder's list of configurations 
    /// 
    /// # Example
    /// ```
    /// 
    /// ```
    pub fn configure(mut self, configuration_name: &str) -> anyhow::Result<()>
    {
        if let None = self.parent.configurator.insert(configuration_name.to_string(), self)
        {
            Ok(())
        }
        else
        {
            Err(NodalAnalysisConfigurationError::ConfigurationNameCollision.into())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodalAnalysisStudyBuilder
{
    pub (in crate) configurator: HashMap<String, NodalAnalysisStudyConfigurator>,
    pub (in crate) model: NodalAnalysisModel, 
}
impl NodalAnalysisStudyBuilder
{
    pub fn new(study_type: &str) -> anyhow::Result<NodalAnalysisStudyBuilder>
    {
        Ok(NodalAnalysisStudyBuilder 
        {
            configurator: HashMap::new(),
            model: NodalAnalysisModel
            {
                model_type: study_type.to_string(),
                nodes: 0,
                configuration: HashMap::new(),
                elements: vec![],
            },
        })
    }

    pub fn add_nodes(mut self, n: usize) -> NodalAnalysisStudyBuilder
    {
        self.model.nodes += n;
        self
    }

    pub fn configure_node(mut self, node: usize, potential: Vec<f64>, is_locked: bool, metadata: Option<HashMap<String, f64>>) -> NodalAnalysisStudyBuilder
    {
        self.model.configuration.insert(node, NodalMetadata { potential, is_locked, metadata });
        self
    }

    pub fn add_element(mut self, element: &str, input: usize, output: usize, gain: Vec<f64>) -> anyhow::Result<NodalAnalysisStudyBuilder>
    {
        if input >= self.model.nodes || output >= self.model.nodes
        { 
            return Err(NodalAnalysisModellingError::NodeDoesNotExist.into());
        };
        self.model.elements.push(
            NodalAnalysisElement { element_type: element.to_string(), input, output, gain, }
        );
        Ok(self)
    }    

    pub fn run_study(self) -> anyhow::Result<NodalAnalysisStudyResult>
    {
        let nodes = vec![GenericNode ; ];


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
