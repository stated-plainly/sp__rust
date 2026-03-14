use crate::enums::eLexeme;
use crate::structs::sToken;
use crate::traits::tLexeme;
use crate::traits::tTokenIdentifier;

pub struct sLexer<TI: tTokenIdentifier> {
    lexemes: Vec<eLexeme<TI>>,
}

impl<TI: tTokenIdentifier> sLexer<TI> {
    pub fn new() -> Self {
        Self {
            lexemes: vec![],
        }
    }

    pub fn add_lexeme(mut self, lexeme: eLexeme<TI>) -> Self {
        self.lexemes.push(lexeme);

        self
    }

    pub fn tokenise(&self, source_code: &str) -> Vec<sToken<TI>> {
        let mut output: Vec<sToken<TI>> = vec![];

        for char_index in 0..source_code.len() {
            for lexeme in self.lexemes.iter() {
                let Some(token) = lexeme.try_tokenise(source_code, char_index) else { continue; };

                output.push(token);
            }
        }

        output
    }
}
