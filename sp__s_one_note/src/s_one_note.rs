use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__e_one_note::eOneNote;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;
use sp__t_note::tNote;

#[derive(Clone)]
pub struct sOneNote {
    letter: eNoteLetter,
    modifier: eNoteModifier,
}

impl sOneNote {
    pub fn new(root: eOneNote) -> Self {
        Self::from(root)
    }
}

impl tNote for sOneNote {
    type EnumNoteVariant = eOneNote;
    
    fn get_letter(&self) -> eNoteLetter {
        self.letter
    }

    fn get_modifier(&self) -> eNoteModifier {
        self.modifier
    }
}

impl Display for sOneNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.letter, self.modifier)
    }
}

impl tKeyboardFriendlyName for sOneNote {
    fn get_keyboard_friendly_name(&self) -> String {
        format!("{}{}", self.letter.get_keyboard_friendly_name(), self.modifier.get_keyboard_friendly_name())
    }
}

impl From<eOneNote> for sOneNote {
    fn from(value: eOneNote) -> Self {
        match value {
            // F
            eOneNote::FFlat => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::FNatural => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::FSharp => {
                return Self {
                    letter: eNoteLetter::F,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // C
            eOneNote::CFlat => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::CNatural => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::CSharp => {
                return Self {
                    letter: eNoteLetter::C,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // G
            eOneNote::GFlat => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::GNatural => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::GSharp => {
                return Self {
                    letter: eNoteLetter::G,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // D
            eOneNote::DFlat => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::DNatural => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::DSharp => {
                return Self {
                    letter: eNoteLetter::D,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // A
            eOneNote::AFlat => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::ANatural => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::ASharp => {
                return Self {
                    letter: eNoteLetter::A,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // E
            eOneNote::EFlat => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::ENatural => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::ESharp => {
                return Self {
                    letter: eNoteLetter::E,
                    modifier: eNoteModifier::Sharp,
                }
            }
            // B
            eOneNote::BFlat => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Flat,
                }
            }
            eOneNote::BNatural => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Natural,
                }
            }
            eOneNote::BSharp => {
                return Self {
                    letter: eNoteLetter::B,
                    modifier: eNoteModifier::Sharp,
                }
            }
        }
    }
}
