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
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use anyhow::Ok;
use modelling::{NodalAnalysisElement, NodalAnalysisModel, NodalMetadata};
// 3rd party modules
use serde::Serialize;
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
use errors::{DroppedNodeError, NodalAnalysisConfigurationError, NodalAnalysisModellingError};
use modelling::element::{ElementConstructor, GenericElement};
use modelling::node::GenericNode;
use serde_json::to_string_pretty;
use ssdc_circuits::{current_source, resistor, voltage_source};

/// The default settings used by the neapolitan solver to build models
pub fn default_study_builder_config() -> HashMap<String, NodalAnalysisStudyConfigurator> 
{
    HashMap::from([
        ("ssdc_circuit".to_string(), NodalAnalysisStudyConfigurator 
        { 
            dimension: 1, 
            elements: HashMap::from([
                ("resistor",        resistor        as ElementConstructor),
                ("voltage_source",  voltage_source  as ElementConstructor),
                ("current_source",  current_source  as ElementConstructor),
            ])
        })
    ])
}

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
    elements: HashMap<String, Vec<f64>>,
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
    dimension: usize,
    elements: HashMap<&'static str, ElementConstructor>,
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
    pub fn add_element_type(mut self, name: &'static str, element_type: ElementConstructor) -> anyhow::Result<NodalAnalysisStudyConfigurator>
    {
        if self.elements.insert(name, element_type).is_none()
        {
            Ok(self)
        }
        else
        {
            Err(NodalAnalysisConfigurationError::ElementTypeNameCollision.into())
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
    pub fn new(study_type: String, configurator: Option<HashMap<String, NodalAnalysisStudyConfigurator>>) -> anyhow::Result<NodalAnalysisStudyBuilder>
    {
        let config_map = match configurator
        {
            None => default_study_builder_config(),
            Some(config) => config,
        };

        if !config_map.contains_key(&study_type)
        {
            return Err(NodalAnalysisModellingError::ModelTypeNotFound.into());
        }

        Ok(NodalAnalysisStudyBuilder 
        {
            configurator: config_map,
            model: NodalAnalysisModel
            {
                model_type: study_type.to_string(),
                nodes: 0,
                configuration: HashMap::new(),
                elements: vec![],
            },
        })
    }

    pub fn from_model_with_default_config(model: NodalAnalysisModel) -> NodalAnalysisStudyBuilder
    {
        NodalAnalysisStudyBuilder
        {
            configurator: default_study_builder_config(),
            model,
        }
    }

    fn get_element_constructor(&self, elem: &str) -> ElementConstructor
    {
        let configurator = &self.configurator[&self.model.model_type];
        configurator.elements[elem]
    }

    fn get_dimension(&self) -> usize
    {
        let configurator = &self.configurator[&self.model.model_type];
        configurator.dimension
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

    pub fn save_model(self, model_rep: &mut String) -> anyhow::Result<NodalAnalysisStudyBuilder>
    {
        let res = to_string_pretty(&self.model);
        if let Err(e) = res
        {
            return Err(e.into());
        }
        *model_rep = res.unwrap();
        Ok(self)
    }

    pub fn run_study(self, margin: f64, limit: usize) -> anyhow::Result<NodalAnalysisStudyResult>
    {
        let n = self.get_dimension();
        let mut nodes = vec![];
        let mut elements = vec![];

        // Step 1 - create/initialize nodes for model
        for _ in 0..self.model.nodes
        {
            nodes.push(
                Rc::new(RefCell::new(GenericNode 
                {
                    potential: Matrix::from_col_vec(vec![1.0; n]),
                    inputs: vec![],
                    outputs: vec![],
                    is_locked: false,
                    _metadata: None,
                }))
            );
        }

        // Step 2 - set nodal metadata if it is given
        for (&i, node_data) in &self.model.configuration
        {
            let mut node = nodes[i].borrow_mut();
            node.potential = Matrix::from_col_vec(node_data.potential.to_vec());
            node.is_locked = node_data.is_locked;
            node._metadata = node_data.metadata.clone();
        }

        // Step 3 - build model 
        for element_data in &self.model.elements
        {
            let NodalAnalysisElement { element_type, input, output, gain } = element_data;
            let constructor = self.get_element_constructor(element_type);
            elements.push(constructor(
                Rc::downgrade(&nodes[*input]), 
                Rc::downgrade(&nodes[*output]), 
                gain.to_vec(),
            )?);
        }

        // Step 4 - solve model
        let mut partials = vec![];
        let mut guess = HashMap::new();
        for (node_idx, _) in nodes.iter().enumerate().filter(|(_, x)| !x.borrow().is_locked)
        {
            for comp_idx in 0..self.get_dimension()
            {
                let idx = ComponentIndex 
                { 
                    node: node_idx as u32, 
                    component: comp_idx as u32 
                };

                let local_nodes = nodes.to_vec();

                guess.insert(idx, 1.0);
                partials.push(move |x: &HashMap<ComponentIndex, f64>| {
                    for (&ComponentIndex { node, component }, &val) in x
                    {
                        local_nodes[node as usize]
                            .try_borrow_mut()?
                            .potential[(component as usize, 0)] = val;
                    }

                    let flux_discrepancy = local_nodes[node_idx]
                        .try_borrow()?
                        .get_flux_discrepancy()?;

                    Ok(flux_discrepancy[(comp_idx, 0)])
                });
            }
        }

        let soln = multivariate_newton_raphson(partials, &mut guess, margin, limit)?;

        // Step 5 - Set model state to solution
        for (idx, component) in soln
        {
            let mut node = nodes[idx.node as usize].try_borrow_mut()?;
            node.potential[(idx.component as usize, 0)] = *component;
        }

        // Step 6 - gather results
        let mut result = NodalAnalysisStudyResult 
        { 
            nodes: HashMap::new(), 
            elements: HashMap::new() 
        };
        
        for (idx, elem) in elements.iter().enumerate()
        {
            result.elements.insert(
                format!("{}.{idx}", self.model.elements[idx].element_type),
                elem.get_flux()?.into()
            );
        }

        // Get all nodal potential values for solution
        for (idx, node) in nodes.iter().enumerate()
        {
            result.nodes.insert(
                idx as u32, 
                node.try_borrow()?.potential.clone().into(),
            );
        }

        Ok(result)
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
