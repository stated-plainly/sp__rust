use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__e_two_note::eTwoNote;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;
use sp__t_note::tNote;

#[derive(Clone)]
pub struct sTwoNote {
    letter: eNoteLetter,
    modifier: eNoteModifier,
}

impl sTwoNote {
    pub fn new(root: eTwoNote) -> Self {
        Self::from(root)
    }
}

impl tNote for sTwoNote {
    type EnumNoteVariant = eTwoNote;

    fn get_letter(&self) -> eNoteLetter {
        self.letter
    }

    fn get_modifier(&self) -> eNoteModifier {
        self.modifier
    }
}

impl Display for sTwoNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.modifier)
    }
}

impl tKeyboardFriendlyName for sTwoNote {
    fn get_keyboard_friendly_name(&self) -> String {
        format!("{}{}", self.letter.get_keyboard_friendly_name(), self.modifier.get_keyboard_friendly_name())
    }
}

impl From<eTwoNote> for sTwoNote {
    fn from(value: eTwoNote) -> Self {
        match value {
            // G
            eTwoNote::GFlat => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoNote::GNatural => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::GSharp => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // D
            eTwoNote::DFlat => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoNote::DNatural => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::DSharp => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // A
            eTwoNote::AFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoNote::ANatural => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::ASharp => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // E
            eTwoNote::EFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoNote::ENatural => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::ESharp => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // B
            eTwoNote::BFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoNote::BNatural => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::BSharp => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // F
            eTwoNote::FNatural => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::FSharp => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Sharp,
                }
            }
            eTwoNote::FDoubleSharp => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::DoubleSharp,
                }
            }
            // C
            eTwoNote::CNatural => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoNote::CSharp => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Sharp,
                }
            }
            eTwoNote::CDoubleSharp => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::DoubleSharp,
                }
            }
        }
    }
}
