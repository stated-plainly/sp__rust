use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::structs::sPatternLexeme;
use crate::structs::sToken;
use crate::structs::sValueLexeme;
use crate::traits::tLexeme;
use crate::traits::tTokenIdentifier;

pub enum eLexeme<TI: tTokenIdentifier> {
    Value(sValueLexeme<TI>),
    Pattern(sPatternLexeme<TI>),
}

impl<TI: tTokenIdentifier> tLexeme<TI> for eLexeme<TI> {
    fn try_tokenise(&self, source_code: &str, char_index: usize) -> Option<sToken<TI>> {
        match self {
            Self::Value(value_lexeme) => value_lexeme.try_tokenise(source_code, char_index),
            Self::Pattern(pattern_lexeme) => pattern_lexeme.try_tokenise(source_code, char_index),
        }
    }
}

impl<TI: tTokenIdentifier> Display for eLexeme<TI> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Value(value_lexeme) => value_lexeme.fmt(f),
            Self::Pattern(pattern_lexeme) => pattern_lexeme.fmt(f),
        }
    }
}
