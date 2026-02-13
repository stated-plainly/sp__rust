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

impl eNoteLetter {
    pub fn prev(&self) -> Self {
        match self {
            Self::C => Self::B,
            Self::D => Self::C,
            Self::E => Self::D,
            Self::F => Self::E,
            Self::G => Self::F,
            Self::A => Self::G,
            Self::B => Self::A,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Self::C => Self::D,
            Self::D => Self::E,
            Self::E => Self::F,
            Self::F => Self::G,
            Self::G => Self::A,
            Self::A => Self::B,
            Self::B => Self::C,
        }
    }
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
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
            Self::C => "c",
            Self::D => "d",
            Self::E => "e",
            Self::F => "f",
            Self::G => "g",
            Self::A => "a",
            Self::B => "b",
        }.to_string()
    }
}
