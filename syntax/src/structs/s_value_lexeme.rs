use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use identifier::traits::tIdentifier;

use crate::structs::sToken;
use crate::traits::tLexeme;

pub struct sValueLexeme<I: tIdentifier> {
    identifier: I,
    value: String,
}

impl<I: tIdentifier> sValueLexeme<I> {
    pub fn new(identifier: I, value: &str) -> Self {
        Self {
            identifier,
            value: value.to_string(),
        }
    }

    pub fn get_identifier(&self) -> I {
        self.identifier
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl<I: tIdentifier> tLexeme<I> for sValueLexeme<I> {
    fn try_tokenise(&self, source_code: &str, char_index: usize) -> Option<sToken<I>> {
        let source_code_slice = &source_code[char_index..];

        if !source_code_slice.starts_with(self.get_value()) { return None; }

        Some(sToken::new(self.get_identifier(), self.get_value()))
    }
}

impl<I: tIdentifier> Display for sValueLexeme<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ValueLexeme({} :: \"{}\")", self.get_identifier(), self.get_value())
    }
}
