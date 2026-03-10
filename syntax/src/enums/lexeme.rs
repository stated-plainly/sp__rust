use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use identifier::traits::tIdentifier;

use crate::structs::sPatternLexeme;
use crate::structs::sToken;
use crate::structs::sValueLexeme;
use crate::traits::tLexeme;

pub enum eLexeme<I: tIdentifier> {
    Value(sValueLexeme<I>),
    Pattern(sPatternLexeme<I>),
}

impl<I: tIdentifier> tLexeme<I> for eLexeme<I> {
    fn try_tokenise(&self, source_code: &str, char_index: usize) -> Option<sToken<I>> {
        match self {
            Self::Value(value_lexeme) => value_lexeme.try_tokenise(source_code, char_index),
            Self::Pattern(pattern_lexeme) => pattern_lexeme.try_tokenise(source_code, char_index),
        }
    }
}

impl<I: tIdentifier> Display for eLexeme<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Value(value_lexeme) => value_lexeme.fmt(f),
            Self::Pattern(pattern_lexeme) => pattern_lexeme.fmt(f),
        }
    }
}
