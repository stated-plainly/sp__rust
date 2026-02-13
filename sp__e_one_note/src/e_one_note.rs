use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eOneNote {
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

impl Display for eOneNote {
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

impl tKeyboardFriendlyName for eOneNote {
    fn get_keyboard_friendly_name(&self) -> String {
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
        }.to_string()
    }
}

impl TryFrom<(eNoteLetter, eNoteModifier)> for eOneNote {
    type Error = Box<dyn tError>;

    fn try_from(value: (eNoteLetter, eNoteModifier)) -> Result<Self, Self::Error> {
        match value {
            // C
            (eNoteLetter::C, eNoteModifier::Flat) => Ok(Self::CFlat),
            (eNoteLetter::C, eNoteModifier::Natural) => Ok(Self::C),
            (eNoteLetter::C, eNoteModifier::Sharp) => Ok(Self::CSharp),
            // D
            (eNoteLetter::D, eNoteModifier::Flat) => Ok(Self::DFlat),
            (eNoteLetter::D, eNoteModifier::Natural) => Ok(Self::D),
            (eNoteLetter::D, eNoteModifier::Sharp) => Ok(Self::DSharp),
            // E
            (eNoteLetter::E, eNoteModifier::Flat) => Ok(Self::EFlat),
            (eNoteLetter::E, eNoteModifier::Natural) => Ok(Self::E),
            (eNoteLetter::E, eNoteModifier::Sharp) => Ok(Self::ESharp),
            // F
            (eNoteLetter::F, eNoteModifier::Flat) => Ok(Self::FFlat),
            (eNoteLetter::F, eNoteModifier::Natural) => Ok(Self::F),
            (eNoteLetter::F, eNoteModifier::Sharp) => Ok(Self::FSharp),
            // G
            (eNoteLetter::G, eNoteModifier::Flat) => Ok(Self::GFlat),
            (eNoteLetter::G, eNoteModifier::Natural) => Ok(Self::G),
            (eNoteLetter::G, eNoteModifier::Sharp) => Ok(Self::GSharp),
            // A
            (eNoteLetter::A, eNoteModifier::Flat) => Ok(Self::AFlat),
            (eNoteLetter::A, eNoteModifier::Natural) => Ok(Self::A),
            (eNoteLetter::A, eNoteModifier::Sharp) => Ok(Self::ASharp),
            // B
            (eNoteLetter::B, eNoteModifier::Flat) => Ok(Self::BFlat),
            (eNoteLetter::B, eNoteModifier::Natural) => Ok(Self::B),
            (eNoteLetter::B, eNoteModifier::Sharp) => Ok(Self::BSharp),
            // Failure
            (note, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("{}{} cannot be converted to a 1 note", note, modifier).as_str()))),
        }
    }
}
