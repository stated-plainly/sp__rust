use regex::exports::Regex;

use syntax::exports::eLexeme;
use syntax::exports::sLexer;
use syntax::exports::sPatternLexeme;
use syntax::exports::sValueLexeme;

use crate::lexer::eTokenIdentifier;

pub fn f_build_lexer() -> sLexer<eTokenIdentifier> {
    sLexer::new()
        .add_lexeme(eLexeme::Pattern(sPatternLexeme::new(eTokenIdentifier::LowerCaseLetter, Regex::new(r"^[a-z]").unwrap())))
        .add_lexeme(eLexeme::Pattern(sPatternLexeme::new(eTokenIdentifier::Number, Regex::new(r"^[0-9]").unwrap())))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::Underscore, "_")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::Colon, ":")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::Whitespace, " ")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::ParenOpen, "(")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::ParenClose, ")")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::SquareOpen, "[")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::SquareClose, "]")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::CurlyOpen, "{")))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::CurlyClose, "}")))
        .add_lexeme(eLexeme::Pattern(sPatternLexeme::new(eTokenIdentifier::Newline, Regex::new(r"^\r?\n").unwrap())))
        .add_lexeme(eLexeme::Value(sValueLexeme::new(eTokenIdentifier::Tab, "\t")))
        .add_lexeme(eLexeme::Pattern(sPatternLexeme::new(eTokenIdentifier::Other, Regex::new(r"^*").unwrap())))
}
