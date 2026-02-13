use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eSevenFlatNote {
    // E
    EDoubleFlat,
    EFlat,
    ENatural,
    // B
    BDoubleFlat,
    #[default]
    BFlat,
    BNatural,
    // F
    FFlat,
    FNatural,
    FSharp,
    // C
    CDoubleFlat,
    CFlat,
    CNatural,
    // G
    GFlat,
    GNatural,
    GSharp,
    // D
    DFlat,
    DNatural,
    DSharp,
    // A
    AFlat,
    ANatural,
    ASharp,
}

impl Display for eSevenFlatNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // E
            Self::EDoubleFlat => write!(f, "{}", "E𝄫"),
            Self::EFlat => write!(f, "{}", "E♭"),
            Self::ENatural => write!(f, "{}", "E♮"),
            // B
            Self::BDoubleFlat => write!(f, "{}", "B𝄫"),
            Self::BFlat => write!(f, "{}", "B♭"),
            Self::BNatural => write!(f, "{}", "B♮"),
            // F
            Self::FFlat => write!(f, "{}", "F♭"),
            Self::FNatural => write!(f, "{}", "F♮"),
            Self::FSharp => write!(f, "{}", "F♯"),
            // C
            Self::CDoubleFlat => write!(f, "{}", "C𝄫"),
            Self::CFlat => write!(f, "{}", "C♭"),
            Self::CNatural => write!(f, "{}", "C♮"),
            // G
            Self::GFlat => write!(f, "{}", "G♭"),
            Self::GNatural => write!(f, "{}", "G♮"),
            Self::GSharp => write!(f, "{}", "G♯"),
            // D
            Self::DFlat => write!(f, "{}", "D♭"),
            Self::DNatural => write!(f, "{}", "D♮"),
            Self::DSharp => write!(f, "{}", "D♯"),
            // A
            Self::AFlat => write!(f, "{}", "A♭"),
            Self::ANatural => write!(f, "{}", "A♮"),
            Self::ASharp => write!(f, "{}", "A♯"),
        }
    }
}

impl tKeyboardFriendlyName for eSevenFlatNote {
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
            // E
            Self::EDoubleFlat => "ebb",
            Self::EFlat => "eb",
            Self::ENatural => "e",
            // B
            Self::BDoubleFlat => "bbb",
            Self::BFlat => "bb",
            Self::BNatural => "b",
            // F
            Self::FFlat => "fb",
            Self::FNatural => "f",
            Self::FSharp => "fs",
            // C
            Self::CDoubleFlat => "cbb",
            Self::CFlat => "cb",
            Self::CNatural => "c",
            // G
            Self::GFlat => "gb",
            Self::GNatural => "g",
            Self::GSharp => "gs",
            // D
            Self::DFlat => "db",
            Self::DNatural => "d",
            Self::DSharp => "ds",
            // A
            Self::AFlat => "ab",
            Self::ANatural => "a",
            Self::ASharp => "as",
        }.to_string()
    }
}

impl TryFrom<(eNoteLetter, eNoteModifier)> for eSevenFlatNote {
    type Error = Box<dyn tError>;

    fn try_from(value: (eNoteLetter, eNoteModifier)) -> Result<Self, Self::Error> {
        match value {
            // E
            (eNoteLetter::E, eNoteModifier::DoubleFlat) => Ok(Self::EDoubleFlat),
            (eNoteLetter::E, eNoteModifier::Flat) => Ok(Self::EFlat),
            (eNoteLetter::E, eNoteModifier::Natural) => Ok(Self::ENatural),
            // B
            (eNoteLetter::B, eNoteModifier::DoubleFlat) => Ok(Self::BDoubleFlat),
            (eNoteLetter::B, eNoteModifier::Flat) => Ok(Self::BFlat),
            (eNoteLetter::B, eNoteModifier::Natural) => Ok(Self::BNatural),
            // F
            (eNoteLetter::F, eNoteModifier::Flat) => Ok(Self::FFlat),
            (eNoteLetter::F, eNoteModifier::Natural) => Ok(Self::FNatural),
            (eNoteLetter::F, eNoteModifier::Sharp) => Ok(Self::FSharp),
            // C
            (eNoteLetter::C, eNoteModifier::DoubleFlat) => Ok(Self::CDoubleFlat),
            (eNoteLetter::C, eNoteModifier::Flat) => Ok(Self::CFlat),
            (eNoteLetter::C, eNoteModifier::Natural) => Ok(Self::CNatural),
            // G
            (eNoteLetter::G, eNoteModifier::Flat) => Ok(Self::GFlat),
            (eNoteLetter::G, eNoteModifier::Natural) => Ok(Self::GNatural),
            (eNoteLetter::G, eNoteModifier::Sharp) => Ok(Self::GSharp),
            // D
            (eNoteLetter::D, eNoteModifier::Flat) => Ok(Self::DFlat),
            (eNoteLetter::D, eNoteModifier::Natural) => Ok(Self::DNatural),
            (eNoteLetter::D, eNoteModifier::Sharp) => Ok(Self::DSharp),
            // A
            (eNoteLetter::A, eNoteModifier::Flat) => Ok(Self::AFlat),
            (eNoteLetter::A, eNoteModifier::Natural) => Ok(Self::ANatural),
            (eNoteLetter::A, eNoteModifier::Sharp) => Ok(Self::ASharp),
            // Failure
            (note, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("{}{} cannot be converted to a 7♭ note", note, modifier).as_str()))),
        }
    }
}
