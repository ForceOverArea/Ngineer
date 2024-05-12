
use thiserror::Error;

#[derive(Debug, Error)]
#[error("the element could not be created because both its input and output nodes were already locked.")]
pub struct ElementCreationError;