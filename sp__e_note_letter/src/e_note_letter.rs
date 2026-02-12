use std::fmt::Display;

use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eNoteLetter {
    #[default]
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}


impl Display for eNoteLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "{}", "C"),
            Self::D => write!(f, "{}", "D"),
            Self::E => write!(f, "{}", "E"),
            Self::F => write!(f, "{}", "F"),
            Self::G => write!(f, "{}", "G"),
            Self::A => write!(f, "{}", "A"),
            Self::B => write!(f, "{}", "B"),
        }
    }
}

impl tKeyboardFriendlyName for eNoteLetter {
    fn get_keyboard_friendly_name(&self) -> &'static str {
        match self {
            Self::C => "c",
            Self::D => "d",
            Self::E => "e",
            Self::F => "f",
            Self::G => "g",
            Self::A => "a",
            Self::B => "b",
        }
    }
}
