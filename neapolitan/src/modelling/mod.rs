pub mod element;
pub mod node;

/// Std modules
use std::{collections::HashMap, str::FromStr};

/// 3rd party modules
use serde::de::{Deserialize, IntoDeserializer, value};

/// Local modules
pub use element::GenericElement;
pub use node::GenericNode;

/// Represents an element in a nodal analysis problem.
/// 
/// # Fields:
/// - `element_type` - the kind of element that should be added in the model
/// - `input`, `output` - the nodes to connect to the element's input and output ports, respectively
/// - `gain` - the element's gain value expressed as a list of values
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct NodalAnalysisElement
{
    element_type: String, 
    input: usize,
    output: usize,
    gain: Vec<f64>,
}

/// Represents nodal metadata that should be set during the model's configuration stage
/// 
/// # Fields:
/// - `node` - the node to set the metadata value for
/// - `metadata` - the serde-friendly metadata to set for the node
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct NodalMetadata
{
    node: usize,
    is_locked: bool,
    metadata: HashMap<String, f64>,
}

/// Represents an entire nodal analysis problem
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct NodalAnalysisModel 
{
    model_type: String,
    nodes: usize,
    configuration: Vec<NodalMetadata>,
    elements: Vec<NodalAnalysisElement>,
}
impl FromStr for NodalAnalysisModel
{
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        Self::deserialize(s.into_deserializer())
    }
}