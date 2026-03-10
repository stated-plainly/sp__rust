use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use regex::Regex;

use identifier::traits::tIdentifier;

use crate::structs::sToken;
use crate::traits::tLexeme;

pub struct sPatternLexeme<I: tIdentifier> {
    identifier: I,
    pattern: Regex,
}

impl<I: tIdentifier> sPatternLexeme<I> {
    pub fn new(identifier: I, pattern: Regex) -> Self {
        Self {
            identifier,
            pattern,
        }
    }

    pub fn get_identifier(&self) -> I {
        self.identifier
    }

    pub fn get_pattern(&self) -> Regex {
        self.pattern.clone()
    }
}

impl<I: tIdentifier> tLexeme<I> for sPatternLexeme<I> {
    fn try_tokenise(&self, source_code: &str, index: usize) -> Option<sToken<I>> {
        let source_code_slice = &source_code[index..];

        let Some(captures) = self.pattern.captures(source_code_slice) else { return None };

        Some(sToken::new(self.get_identifier(), &captures[1]))
    }
}

impl<I: tIdentifier> Display for sPatternLexeme<I> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PatternLexeme({} :: /{}/)", self.get_identifier(), self.get_pattern())
    }
}
