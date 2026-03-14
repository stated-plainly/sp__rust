use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use identifier::traits::tIdentifier;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum eTokenIdentifier {
    LowerCaseLetter,
    Number,
    Underscore,
    Colon,
    Whitespace,
    ParenOpen,
    ParenClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
    Newline,
    Tab,
    Other,
}

impl tIdentifier for eTokenIdentifier {
    fn as_vec() -> Vec<eTokenIdentifier> {
        vec![
            Self::LowerCaseLetter,
            Self::Number,
            Self::Underscore,
            Self::Colon,
            Self::Whitespace,
            Self::ParenOpen,
            Self::ParenClose,
            Self::SquareOpen,
            Self::SquareClose,
            Self::CurlyOpen,
            Self::CurlyClose,
            Self::Newline,
            Self::Tab,
            Self::Other,
        ]
    }
}

impl Display for eTokenIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let as_string: String = match self {
            Self::LowerCaseLetter => "Letter<Lower>",
            Self::Number => "Number",
            Self::Underscore => "Underscore",
            Self::Colon => "Colon",
            Self::Whitespace => "Whitespace",
            Self::ParenOpen => "Paren<Open>",
            Self::ParenClose => "Paren<Close>",
            Self::SquareOpen => "Square<Open>",
            Self::SquareClose => "Square<Close>",
            Self::CurlyOpen => "Curly<Open>",
            Self::CurlyClose => "Curly<Close>",
            Self::Newline => "Newline",
            Self::Tab => "Tab",
            Self::Other => "--Other--",
        }.to_string();

        write!(f, "{}", as_string)
    }
}
