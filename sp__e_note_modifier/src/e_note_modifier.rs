use std::fmt::Display;

use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
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

impl eNoteModifier {
    pub fn try_flatten(&self) -> Result<Self, Box<dyn tError>> {
        match self {
            Self::DoubleFlat => Err(Box::new(sImpossibleOperationError::new("Cannot flatten a double-flat note"))),
            Self::Flat => Ok(Self::DoubleFlat),
            Self::Natural => Ok(Self::Flat),
            Self::Sharp => Ok(Self::Natural),
            Self::DoubleSharp => Ok(Self::Sharp),
        }
    }

    pub fn try_sharpen(&self) -> Result<Self, Box<dyn tError>> {
        match self {
            Self::DoubleFlat => Ok(Self::Flat),
            Self::Flat => Ok(Self::Natural),
            Self::Natural => Ok(Self::Sharp),
            Self::Sharp => Ok(Self::DoubleSharp),
            Self::DoubleSharp => Err(Box::new(sImpossibleOperationError::new("Cannot sharpen a double-sharp note"))),
        }
    }
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
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
            Self::DoubleFlat => "bb",
            Self::Flat => "b",
            Self::Natural => "",
            Self::Sharp => "s",
            Self::DoubleSharp => "ss",
        }.to_string()
    }
}
