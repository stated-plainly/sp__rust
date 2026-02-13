use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__e_three_flat_note::eThreeFlatNote;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;
use sp__t_note::tNote;

#[derive(Clone)]
pub struct sThreeFlatNote {
    letter: eNoteLetter,
    modifier: eNoteModifier,
}

impl sThreeFlatNote {
    pub fn new(root: eThreeFlatNote) -> Self {
        Self::from(root)
    }
}

impl tNote for sThreeFlatNote {
    type EnumNoteVariant = eThreeFlatNote;

    fn get_letter(&self) -> eNoteLetter {
        self.letter
    }

    fn get_modifier(&self) -> eNoteModifier {
        self.modifier
    }
}

impl Display for sThreeFlatNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.modifier)
    }
}

impl tKeyboardFriendlyName for sThreeFlatNote {
    fn get_keyboard_friendly_name(&self) -> String {
        format!("{}{}", self.letter.get_keyboard_friendly_name(), self.modifier.get_keyboard_friendly_name())
    }
}

impl From<eThreeFlatNote> for sThreeFlatNote {
    fn from(value: eThreeFlatNote) -> Self {
        match value {
            // A
            eThreeFlatNote::ADoubleFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eThreeFlatNote::AFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::ANatural => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Natural,
                }
            }
            // E
            eThreeFlatNote::EDoubleFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eThreeFlatNote::EFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::ENatural => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Natural,
                }
            }
            // B
            eThreeFlatNote::BDoubleFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::DoubleFlat,
                }
            }
            eThreeFlatNote::BFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::BNatural => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Natural,
                }
            }
            // F
            eThreeFlatNote::FFlat => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::FNatural => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Natural,
                }
            }
            eThreeFlatNote::FSharp => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // C
            eThreeFlatNote::CFlat => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::CNatural => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Natural,
                }
            }
            eThreeFlatNote::CSharp => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // G
            eThreeFlatNote::GFlat => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::GNatural => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Natural,
                }
            }
            eThreeFlatNote::GSharp => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // D
            eThreeFlatNote::DFlat => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Flat,
                }
            }
            eThreeFlatNote::DNatural => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Natural,
                }
            }
            eThreeFlatNote::DSharp => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Sharp,
                }
            }
        }
    }
}
