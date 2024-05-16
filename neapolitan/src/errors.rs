
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