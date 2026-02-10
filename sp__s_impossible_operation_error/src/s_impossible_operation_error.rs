use std::error::Error;
use std::fmt::Display;

use sp__t_error::tError;

#[derive(Debug)]
pub struct sImpossibleOperationError {
    explanation: String,
}

impl sImpossibleOperationError {
    pub fn new(explanation: &str) -> Self {
        Self {
            explanation: explanation.to_string(),
        }
    }
}

impl tError for sImpossibleOperationError {}

impl Error for sImpossibleOperationError {}

impl Display for sImpossibleOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Impossible Operation : {}", self.explanation)
    }
}
