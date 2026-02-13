use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__e_two_flat_note::eTwoFlatNote;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Clone)]
pub struct sTwoFlatNote {
    letter: eNoteLetter,
    modifier: eNoteModifier,
}

impl sTwoFlatNote {
    pub fn new(root: eTwoFlatNote) -> Self {
        Self::from(root)
    }
}

impl Display for sTwoFlatNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.modifier)
    }
}

impl tKeyboardFriendlyName for sTwoFlatNote {
    fn get_keyboard_friendly_name(&self) -> String {
        format!("{}{}", self.letter.get_keyboard_friendly_name(), self.modifier.get_keyboard_friendly_name())
    }
}

impl From<eTwoFlatNote> for sTwoFlatNote {
    fn from(value: eTwoFlatNote) -> Self {
        match value {
            // G
            eTwoFlatNote::GDoubleFlat => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eTwoFlatNote::GFlat => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::GNatural => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Natural,
                }
            }
            // D
            eTwoFlatNote::DDoubleFlat => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eTwoFlatNote::DFlat => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::DNatural => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Natural,
                }
            }
            // A
            eTwoFlatNote::ADoubleFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eTwoFlatNote::AFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::ANatural => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Natural,
                }
            }
            // E
            eTwoFlatNote::EDoubleFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eTwoFlatNote::EFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::ENatural => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Natural,
                }
            }
            // B
            eTwoFlatNote::BDoubleFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eTwoFlatNote::BFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::BNatural => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Natural,
                }
            }
            // F
            eTwoFlatNote::FFlat => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::FNatural => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoFlatNote::FSharp => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // C
            eTwoFlatNote::CFlat => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Flat,
                }
            }
            eTwoFlatNote::CNatural => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Natural,
                }
            }
            eTwoFlatNote::CSharp => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Sharp,
                }
            }
        }
    }
}
