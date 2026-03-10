use error::enums::eError;
use syntax::structs::sToken;

use crate::{ast::sInterLangObject, lexer::eTokenIdentifier};

pub struct sParser {
    char_index: u8,
    line_index: u8,
}

impl sParser {
    pub fn new() -> Self {
        Self {
            char_index: 0,
            line_index: 0,
        }
    }

    pub fn parse(&mut self, tokens: Vec<sToken<eTokenIdentifier>>) -> Result<sInterLangObject, eError> {
        todo!()
    }
}
