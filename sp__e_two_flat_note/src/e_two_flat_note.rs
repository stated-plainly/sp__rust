use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eTwoFlatNote {
    // G
    GDoubleFlat,
    GFlat,
    GNatural,
    // D
    DDoubleFlat,
    #[default]
    DFlat,
    DNatural,
    // A
    ADoubleFlat,
    AFlat,
    ANatural,
    // E
    EDoubleFlat,
    EFlat,
    ENatural,
    // B
    BDoubleFlat,
    BFlat,
    BNatural,
    // F
    FFlat,
    FNatural,
    FSharp,
    // C
    CFlat,
    CNatural,
    CSharp,
}

impl Display for eTwoFlatNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // G
            Self::GDoubleFlat => write!(f, "{}", "G♭♭"),
            Self::GFlat => write!(f, "{}", "G♭"),
            Self::GNatural => write!(f, "{}", "G"),
            // D
            Self::DDoubleFlat => write!(f, "{}", "D♭♭"),
            Self::DFlat => write!(f, "{}", "D♭"),
            Self::DNatural => write!(f, "{}", "D"),
            // A
            Self::ADoubleFlat => write!(f, "{}", "A♭♭"),
            Self::AFlat => write!(f, "{}", "A♭"),
            Self::ANatural => write!(f, "{}", "A"),
            // E
            Self::EDoubleFlat => write!(f, "{}", "E♭♭"),
            Self::EFlat => write!(f, "{}", "E♭"),
            Self::ENatural => write!(f, "{}", "E"),
            // B
            Self::BDoubleFlat => write!(f, "{}", "B♭♭"),
            Self::BFlat => write!(f, "{}", "B♭"),
            Self::BNatural => write!(f, "{}", "B"),
            // F
            Self::FFlat => write!(f, "{}", "F♭"),
            Self::FNatural => write!(f, "{}", "F"),
            Self::FSharp => write!(f, "{}", "F♯"),
            // C
            Self::CFlat => write!(f, "{}", "C♭"),
            Self::CNatural => write!(f, "{}", "C"),
            Self::CSharp => write!(f, "{}", "C♯"),
        }
    }
}

impl tKeyboardFriendlyName for eTwoFlatNote {
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
            // G
            Self::GDoubleFlat => "gbb",
            Self::GFlat => "gb",
            Self::GNatural => "g",
            // D
            Self::DDoubleFlat => "dbb",
            Self::DFlat => "db",
            Self::DNatural => "d",
            // A
            Self::ADoubleFlat => "abb",
            Self::AFlat => "ab",
            Self::ANatural => "a",
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
            Self::CFlat => "cb",
            Self::CNatural => "c",
            Self::CSharp => "cs",
        }.to_string()
    }
}

impl TryFrom<(eNoteLetter, eNoteModifier)> for eTwoFlatNote {
    type Error = Box<dyn tError>;

    fn try_from(value: (eNoteLetter, eNoteModifier)) -> Result<Self, Self::Error> {
        match value {
            // G
            (eNoteLetter::G, eNoteModifier::DoubleFlat) => Ok(Self::GDoubleFlat),
            (eNoteLetter::G, eNoteModifier::Flat) => Ok(Self::GFlat),
            (eNoteLetter::G, eNoteModifier::Natural) => Ok(Self::GNatural),
            // D
            (eNoteLetter::D, eNoteModifier::DoubleFlat) => Ok(Self::DDoubleFlat),
            (eNoteLetter::D, eNoteModifier::Flat) => Ok(Self::DFlat),
            (eNoteLetter::D, eNoteModifier::Natural) => Ok(Self::DNatural),
            // A
            (eNoteLetter::A, eNoteModifier::DoubleFlat) => Ok(Self::ADoubleFlat),
            (eNoteLetter::A, eNoteModifier::Flat) => Ok(Self::AFlat),
            (eNoteLetter::A, eNoteModifier::Natural) => Ok(Self::ANatural),
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
            (eNoteLetter::C, eNoteModifier::Flat) => Ok(Self::CFlat),
            (eNoteLetter::C, eNoteModifier::Natural) => Ok(Self::CNatural),
            (eNoteLetter::C, eNoteModifier::Sharp) => Ok(Self::CSharp),
            // Failure
            (note, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("{}{} cannot be converted to a 1 note", note, modifier).as_str()))),
        }
    }
}
