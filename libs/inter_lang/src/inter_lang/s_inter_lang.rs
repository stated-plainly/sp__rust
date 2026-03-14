use syntax::structs::sLexer;

use crate::ast::sInterLangObject;
use crate::lexer::eTokenIdentifier;
use crate::lexer::f_build_lexer;
use crate::parser::sParser;

pub struct sInterLang {
    lexer: sLexer<eTokenIdentifier>, 
    parser: sParser,
}

impl sInterLang {
    pub fn new() -> Self {
        Self {
            lexer: f_build_lexer(),
            parser: sParser::new(),
        }
    }

    pub fn parse(&mut self, source_code: &str) -> sInterLangObject {
        self.parser.parse(&mut self.lexer.tokenise(source_code))
    }
}
