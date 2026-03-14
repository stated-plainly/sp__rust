use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::structs::sToken;
use crate::traits::tLexeme;
use crate::traits::tTokenIdentifier;

pub struct sValueLexeme<TI: tTokenIdentifier> {
    identifier: TI,
    value: String,
}

impl<TI: tTokenIdentifier> sValueLexeme<TI> {
    pub fn new(identifier: TI, value: &str) -> Self {
        Self {
            identifier,
            value: value.to_string(),
        }
    }

    pub fn get_identifier(&self) -> TI {
        self.identifier
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl<TI: tTokenIdentifier> tLexeme<TI> for sValueLexeme<TI> {
    fn try_tokenise(&self, source_code: &str, char_index: usize) -> Option<sToken<TI>> {
        let source_code_slice = &source_code[char_index..];

        if !source_code_slice.starts_with(self.get_value()) { return None; }

        Some(sToken::new(self.get_identifier(), self.get_value()))
    }
}

impl<TI: tTokenIdentifier> Display for sValueLexeme<TI> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ValueLexeme({} :: \"{}\")", self.get_identifier(), self.get_value())
    }
}
