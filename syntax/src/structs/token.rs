use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use identifier::traits::tIdentifier;

pub struct sToken<I: tIdentifier> {
    identifier: I,
    value: String,
}

impl<I: tIdentifier> sToken<I> {
    pub fn new(identifier: I, value: &str) -> Self {
        Self {
            identifier,
            value: value.to_string(),
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_identifier(&self) -> I {
        self.identifier
    }
}

impl<I: tIdentifier> Display for sToken<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token({} :: \"{}\")", self.get_identifier(), self.get_value())
    }
}
