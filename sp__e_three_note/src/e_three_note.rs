use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eThreeNote {
    // A
    AFlat,
    ANatural,
    ASharp,
    // E
    EFlat,
    #[default]
    ENatural,
    ESharp,
    // B
    BFlat,
    BNatural,
    BSharp,
    // F
    FNatural,
    FSharp,
    FDoubleSharp,
    // C
    CNatural,
    CSharp,
    CDoubleSharp,
    // G
    GNatural,
    GSharp,
    GDoubleSharp,
    // D
    DNatural,
    DSharp,
    DDoubleSharp,
}

impl Display for eThreeNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // A
            Self::AFlat => write!(f, "{}", "A♭"),
            Self::ANatural => write!(f, "{}", "A♮"),
            Self::ASharp => write!(f, "{}", "A♯"),
            // E
            Self::EFlat => write!(f, "{}", "E♭"),
            Self::ENatural => write!(f, "{}", "E♮"),
            Self::ESharp => write!(f, "{}", "E♯"),
            // B
            Self::BFlat => write!(f, "{}", "B♭"),
            Self::BNatural => write!(f, "{}", "B♮"),
            Self::BSharp => write!(f, "{}", "B♯"),
            // F
            Self::FNatural => write!(f, "{}", "F♮"),
            Self::FSharp => write!(f, "{}", "F♯"),
            Self::FDoubleSharp => write!(f, "{}", "F𝄪"),
            // C
            Self::CNatural => write!(f, "{}", "C♮"),
            Self::CSharp => write!(f, "{}", "C♯"),
            Self::CDoubleSharp => write!(f, "{}", "C𝄪"),
            // G
            Self::GNatural => write!(f, "{}", "G♮"),
            Self::GSharp => write!(f, "{}", "G♯"),
            Self::GDoubleSharp => write!(f, "{}", "G𝄪"),
            // D
            Self::DNatural => write!(f, "{}", "D♮"),
            Self::DSharp => write!(f, "{}", "D♯"),
            Self::DDoubleSharp => write!(f, "{}", "D𝄪"),
        }
    }
}

impl tKeyboardFriendlyName for eThreeNote {
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
            // A
            Self::AFlat => "ab",
            Self::ANatural => "a",
            Self::ASharp => "as",
            // E
            Self::EFlat => "eb",
            Self::ENatural => "e",
            Self::ESharp => "es",
            // B
            Self::BFlat => "bb",
            Self::BNatural => "b",
            Self::BSharp => "bs",
            // F
            Self::FNatural => "f",
            Self::FSharp => "fs",
            Self::FDoubleSharp => "fss",
            // C
            Self::CNatural => "c",
            Self::CSharp => "cs",
            Self::CDoubleSharp => "css",
            // G
            Self::GNatural => "g",
            Self::GSharp => "gs",
            Self::GDoubleSharp => "gss",
            // D
            Self::DNatural => "d",
            Self::DSharp => "ds",
            Self::DDoubleSharp => "dss",
        }.to_string()
    }
}

impl TryFrom<(eNoteLetter, eNoteModifier)> for eThreeNote {
    type Error = Box<dyn tError>;

    fn try_from(value: (eNoteLetter, eNoteModifier)) -> Result<Self, Self::Error> {
        match value {
            // A
            (eNoteLetter::A, eNoteModifier::Flat) => Ok(Self::AFlat),
            (eNoteLetter::A, eNoteModifier::Natural) => Ok(Self::ANatural),
            (eNoteLetter::A, eNoteModifier::Sharp) => Ok(Self::ASharp),
            // E
            (eNoteLetter::E, eNoteModifier::Flat) => Ok(Self::EFlat),
            (eNoteLetter::E, eNoteModifier::Natural) => Ok(Self::ENatural),
            (eNoteLetter::E, eNoteModifier::Sharp) => Ok(Self::ESharp),
            // B
            (eNoteLetter::B, eNoteModifier::Flat) => Ok(Self::BFlat),
            (eNoteLetter::B, eNoteModifier::Natural) => Ok(Self::BNatural),
            (eNoteLetter::B, eNoteModifier::Sharp) => Ok(Self::BSharp),
            // F
            (eNoteLetter::F, eNoteModifier::Natural) => Ok(Self::FNatural),
            (eNoteLetter::F, eNoteModifier::Sharp) => Ok(Self::FSharp),
            (eNoteLetter::F, eNoteModifier::DoubleSharp) => Ok(Self::FDoubleSharp),
            // C
            (eNoteLetter::C, eNoteModifier::Natural) => Ok(Self::CNatural),
            (eNoteLetter::C, eNoteModifier::Sharp) => Ok(Self::CSharp),
            (eNoteLetter::C, eNoteModifier::DoubleSharp) => Ok(Self::CDoubleSharp),
            // G
            (eNoteLetter::G, eNoteModifier::Natural) => Ok(Self::GNatural),
            (eNoteLetter::G, eNoteModifier::Sharp) => Ok(Self::GSharp),
            (eNoteLetter::G, eNoteModifier::DoubleSharp) => Ok(Self::GDoubleSharp),
            // D
            (eNoteLetter::D, eNoteModifier::Natural) => Ok(Self::DNatural),
            (eNoteLetter::D, eNoteModifier::Sharp) => Ok(Self::DSharp),
            (eNoteLetter::D, eNoteModifier::DoubleSharp) => Ok(Self::DDoubleSharp),
            // Failure
            (note, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("{}{} cannot be converted to a 3 note", note, modifier).as_str()))),
        }
    }
}
