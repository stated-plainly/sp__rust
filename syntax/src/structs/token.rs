use std::fmt::Display;

use crate::traits::tLexeme;

pub struct sToken<L: tLexeme> {
    lexeme: L,
    value: String,
}

impl<L: tLexeme> sToken<L> {
    pub fn new(lexeme: L, value: &str) -> Self {
        Self {
            lexeme,
            value: value.to_string(),
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_lexeme(&self) -> L {
        self.lexeme
    }
}

impl<L: tLexeme> Display for sToken<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({} :: \"{}\")", self.get_lexeme(), self.get_value())
    }
}
