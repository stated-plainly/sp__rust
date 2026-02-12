use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__e_root_note::eRootNote;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

pub struct sRootNote {
    letter: eNoteLetter,
    modifier: eNoteModifier,
}

impl sRootNote {
    pub fn new(root: eRootNote) -> Self {
        Self::from(root)
    }
}

impl Display for sRootNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.modifier)
    }
}

impl tKeyboardFriendlyName for sRootNote {
    fn get_keyboard_friendly_name(&self) -> String {
        format!("{}{}", self.letter.get_keyboard_friendly_name(), self.modifier.get_keyboard_friendly_name())
    }
}

impl From<eRootNote> for sRootNote {
    fn from(value: eRootNote) -> Self {
        match value {
            // C
            eRootNote::CFlat => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::C => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::CSharp => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // D
            eRootNote::DFlat => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::D => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::DSharp => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // E
            eRootNote::EFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::E => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::ESharp => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // F
            eRootNote::FFlat => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::F => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::FSharp => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // G
            eRootNote::GFlat => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::G => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::GSharp => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // A
            eRootNote::AFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::A => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::ASharp => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // B
            eRootNote::BFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Flat,
                }
            }
            eRootNote::B => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Natural,
                }
            }
            eRootNote::BSharp => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Sharp,
                }
            }
        }
    }
}
