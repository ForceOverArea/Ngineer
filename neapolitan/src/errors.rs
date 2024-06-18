
use thiserror::Error;

#[derive(Debug, Error)]
#[error("the element could not be created because both its input and output nodes were already locked.")]
pub struct ElementCreationError;

#[derive(Debug, Error)]
pub enum EquationGenerationError
{
    #[error("the system equations could not be generated because there were more than 4,294,967,295 nodes in the given model")]
    NodeCountIntegerOverflow,
    #[error("the system equations could not be generated because there were no nodes in the system")]
    NoNodesInSystem,
}

#[derive(Debug, Error)]
pub enum FluxCalculationError
{
    #[error("failed to access nodes during flux calculation because they were already dropped.")]
    NodeRefsAlreadyDropped,
}

#[derive(Debug, Error)]
#[error("node reference was already dropped before attempting to borrow from refcell")]
pub struct DroppedNodeError;

#[derive(Debug, Error)]
pub enum NodalAnalysisConfigurationError
{
    #[error("element type with this name was already created for this configurator object")]
    ElementTypeNameCollision,
    #[error("a configuration with this name was already added to this model builder")]
    ConfigurationNameCollision,
}

#[derive(Debug, Error)]
pub enum NodalAnalysisModellingError
{
    #[error("could not attach element to one or more of the given nodes because the node(s) did not exist in the model")]
    NodeDoesNotExist,
    #[error("could not find desired model type in the given or default configurators")]
    ModelTypeNotFound,
}