use std::fmt::Display;

use crate::traits::tTokenIdentifier;

pub struct sToken<TI: tTokenIdentifier> {
    identifier: TI,
    value: String,
}

impl<TI: tTokenIdentifier> sToken<TI> {
    pub fn new(lexeme: TI, value: &str) -> Self {
        Self {
            identifier: lexeme,
            value: value.to_string(),
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_identifier(&self) -> TI {
        self.identifier
    }
}

impl<TI: tTokenIdentifier> Display for sToken<TI> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({} :: \"{}\")", self.get_identifier(), self.get_value())
    }
}
