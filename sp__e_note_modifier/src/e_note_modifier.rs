use std::fmt::Display;

use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eNoteModifier {
    DoubleFlat,
    Flat,
    #[default]
    Natural,
    Sharp,
    DoubleSharp,
}


impl Display for eNoteModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoubleFlat => write!(f, "{}", "𝄫"),
            Self::Flat => write!(f, "{}", "♭"),
            Self::Natural => write!(f, "{}", "♮"),
            Self::Sharp => write!(f, "{}", "♯"),
            Self::DoubleSharp => write!(f, "{}", "𝄪"),
        }
    }
}

impl tKeyboardFriendlyName for eNoteModifier {
    fn get_keyboard_friendly_name(&self) -> &'static str {
        match self {
            Self::DoubleFlat => "bb",
            Self::Flat => "b",
            Self::Natural => "",
            Self::Sharp => "s",
            Self::DoubleSharp => "ss",
        }
    }
}
