use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use regex::exports::Regex;

use crate::structs::sToken;
use crate::traits::tLexeme;
use crate::traits::tTokenIdentifier;

pub struct sPatternLexeme<TI: tTokenIdentifier> {
    identifier: TI,
    pattern: Regex,
}

impl<TI: tTokenIdentifier> sPatternLexeme<TI> {
    pub fn new(identifier: TI, pattern: Regex) -> Self {
        Self {
            identifier,
            pattern,
        }
    }

    pub fn get_identifier(&self) -> TI {
        self.identifier
    }

    pub fn get_pattern(&self) -> Regex {
        self.pattern.clone()
    }
}

impl<TI: tTokenIdentifier> tLexeme<TI> for sPatternLexeme<TI> {
    fn try_tokenise(&self, source_code: &str, index: usize) -> Option<sToken<TI>> {
        let source_code_slice = &source_code[index..];

        let Some(captures) = self.pattern.captures(source_code_slice) else { return None };

        Some(sToken::new(self.get_identifier(), &captures[1]))
    }
}

impl<TI: tTokenIdentifier> Display for sPatternLexeme<TI> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PatternLexeme({} :: /{}/)", self.get_identifier(), self.get_pattern())
    }
}
