use std::fmt::Display;

use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eRootNote {
    // C
    CFlat,
    #[default]
    C,
    CSharp,
    // D
    DFlat,
    D,
    DSharp,
    // E
    EFlat,
    E,
    ESharp,
    // F
    FFlat,
    F,
    FSharp,
    // G
    GFlat,
    G,
    GSharp,
    // A
    AFlat,
    A,
    ASharp,
    // B
    BFlat,
    B,
    BSharp,
}

impl Display for eRootNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // C
            Self::CFlat => write!(f, "{}", "C♭"),
            Self::C => write!(f, "{}", "C"),
            Self::CSharp => write!(f, "{}", "C♯"),
            // C
            Self::DFlat => write!(f, "{}", "D♭"),
            Self::D => write!(f, "{}", "D"),
            Self::DSharp => write!(f, "{}", "D♯"),
            // E
            Self::EFlat => write!(f, "{}", "E♭"),
            Self::E => write!(f, "{}", "E"),
            Self::ESharp => write!(f, "{}", "E♯"),
            // F
            Self::FFlat => write!(f, "{}", "F♭"),
            Self::F => write!(f, "{}", "F"),
            Self::FSharp => write!(f, "{}", "F♯"),
            // G
            Self::GFlat => write!(f, "{}", "G♭"),
            Self::G => write!(f, "{}", "G"),
            Self::GSharp => write!(f, "{}", "G♯"),
            // A
            Self::AFlat => write!(f, "{}", "A♭"),
            Self::A => write!(f, "{}", "A"),
            Self::ASharp => write!(f, "{}", "A♯"),
            // B
            Self::BFlat => write!(f, "{}", "B♭"),
            Self::B => write!(f, "{}", "B"),
            Self::BSharp => write!(f, "{}", "B♯"),
        }
    }
}

impl tKeyboardFriendlyName for eRootNote {
    fn get_keyboard_friendly_name(&self) -> &'static str {
        match self {
            // C
            Self::CFlat => "cb",
            Self::C => "c",
            Self::CSharp => "cs",
            // C
            Self::DFlat => "db",
            Self::D => "d",
            Self::DSharp => "ds",
            // E
            Self::EFlat => "eb",
            Self::E => "e",
            Self::ESharp => "es",
            // F
            Self::FFlat => "fb",
            Self::F => "f",
            Self::FSharp => "fs",
            // G
            Self::GFlat => "gb",
            Self::G => "g",
            Self::GSharp => "gs",
            // A
            Self::AFlat => "ab",
            Self::A => "a",
            Self::ASharp => "as",
            // B
            Self::BFlat => "bb",
            Self::B => "b",
            Self::BSharp => "bs",
        }
    }
}
