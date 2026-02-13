use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eFiveSharpNote {
    // C
    CNatural,
    CSharp,
    CDoubleSharp,
    // G
    GNatural,
    #[default]
    GSharp,
    GDoubleSharp,
    // D
    DNatural,
    DSharp,
    DDoubleSharp,
    // A
    ANatural,
    ASharp,
    ADoubleSharp,
    // E
    ENatural,
    ESharp,
    EDoubleSharp,
    // B
    BNatural,
    BSharp,
    BDoubleSharp,
    // F
    FSharp,
    FDoubleSharp,
    FTripleSharp,
}

impl Display for eFiveSharpNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            // A
            Self::ANatural => write!(f, "{}", "A♮"),
            Self::ASharp => write!(f, "{}", "A♯"),
            Self::ADoubleSharp => write!(f, "{}", "A𝄪"),
            // E
            Self::ENatural => write!(f, "{}", "E♮"),
            Self::ESharp => write!(f, "{}", "E♯"),
            Self::EDoubleSharp => write!(f, "{}", "E𝄪"),
            // B
            Self::BNatural => write!(f, "{}", "B♮"),
            Self::BSharp => write!(f, "{}", "B♯"),
            Self::BDoubleSharp => write!(f, "{}", "B𝄪"),
            // F
            Self::FSharp => write!(f, "{}", "F♯"),
            Self::FDoubleSharp => write!(f, "{}", "F𝄪"),
            Self::FTripleSharp => write!(f, "{}", "F♯𝄪"),
        }
    }
}

impl tKeyboardFriendlyName for eFiveSharpNote {
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
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
            // A
            Self::ANatural => "a",
            Self::ASharp => "as",
            Self::ADoubleSharp => "ass",
            // E
            Self::ENatural => "e",
            Self::ESharp => "es",
            Self::EDoubleSharp => "ess",
            // B
            Self::BNatural => "b",
            Self::BSharp => "bs",
            Self::BDoubleSharp => "bss",
            // F
            Self::FSharp => "fs",
            Self::FDoubleSharp => "fss",
            Self::FTripleSharp => "fsss",
        }.to_string()
    }
}

impl TryFrom<(eNoteLetter, eNoteModifier)> for eFiveSharpNote {
    type Error = Box<dyn tError>;

    fn try_from(value: (eNoteLetter, eNoteModifier)) -> Result<Self, Self::Error> {
        match value {
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
            // A
            (eNoteLetter::A, eNoteModifier::Natural) => Ok(Self::ANatural),
            (eNoteLetter::A, eNoteModifier::Sharp) => Ok(Self::ASharp),
            (eNoteLetter::A, eNoteModifier::DoubleSharp) => Ok(Self::ADoubleSharp),
            // E
            (eNoteLetter::E, eNoteModifier::Natural) => Ok(Self::ENatural),
            (eNoteLetter::E, eNoteModifier::Sharp) => Ok(Self::ESharp),
            (eNoteLetter::E, eNoteModifier::DoubleSharp) => Ok(Self::EDoubleSharp),
            // B
            (eNoteLetter::B, eNoteModifier::Natural) => Ok(Self::BNatural),
            (eNoteLetter::B, eNoteModifier::Sharp) => Ok(Self::BSharp),
            (eNoteLetter::B, eNoteModifier::DoubleSharp) => Ok(Self::BDoubleSharp),
            // F
            (eNoteLetter::F, eNoteModifier::Sharp) => Ok(Self::FSharp),
            (eNoteLetter::F, eNoteModifier::DoubleSharp) => Ok(Self::FDoubleSharp),
            (eNoteLetter::F, eNoteModifier::TripleSharp) => Ok(Self::FTripleSharp),
            // Failure
            (note, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("{}{} cannot be converted to a 5♯ note", note, modifier).as_str()))),
        }
    }
}
