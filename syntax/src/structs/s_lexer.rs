use identifier::traits::tIdentifier;

use crate::enums::eLexeme;
use crate::structs::sToken;
use crate::traits::tLexeme;

pub struct sLexer<I: tIdentifier> {
    lexemes: Vec<eLexeme<I>>,
}

impl<I: tIdentifier> sLexer<I> {
    pub fn new() -> Self {
        Self {
            lexemes: vec![],
        }
    }

    pub fn add_lexeme(mut self, lexeme: eLexeme<I>) -> Self {
        self.lexemes.push(lexeme);

        self
    }

    pub fn tokenise(&self, source_code: &str) -> Vec<sToken<I>> {
        let mut output: Vec<sToken<I>> = vec![];

        for char_index in 0..source_code.len() {
            for lexeme in self.lexemes.iter() {
                let Some(token) = lexeme.try_tokenise(source_code, char_index) else { continue; };

                output.push(token);
            }
        }

        output
    }
}
