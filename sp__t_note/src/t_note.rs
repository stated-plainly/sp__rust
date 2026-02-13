use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;

pub trait tNote : Clone + Display {
    fn get_letter(&self) -> eNoteLetter;

    fn get_modifier(&self) -> eNoteModifier;
}
