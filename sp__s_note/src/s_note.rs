use std::fmt::Display;

use sp__e_note_letter::eNoteLetter;
use sp__e_note_modifier::eNoteModifier;
use sp__e_scale_degree::eScaleDegree;
use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_error::tError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

#[derive(Clone)]
pub struct sNote {
    letter: eNoteLetter,
    modifier: eNoteModifier,
    scale_degree: eScaleDegree,
}

impl sNote {
    pub fn new(letter: eNoteLetter, modifier: eNoteModifier, scale_degree: eScaleDegree) -> Result<Self, Box<dyn tError>> {
        match scale_degree {
            eScaleDegree::One => Self::new_one(letter, modifier),
            eScaleDegree::TwoFlat => Self::new_two_flat(letter, modifier),
            eScaleDegree::Two => Self::new_two(letter, modifier),
            eScaleDegree::ThreeFlat => Self::new_three_flat(letter, modifier),
            eScaleDegree::Three => Self::new_three(letter, modifier),
            eScaleDegree::Four => Self::new_four(letter, modifier),
            eScaleDegree::FourSharp => Self::new_four_sharp(letter, modifier),
            eScaleDegree::FiveFlat => Self::new_five_flat(letter, modifier),
            eScaleDegree::Five => Self::new_five(letter, modifier),
            eScaleDegree::FiveSharp => Self::new_five_sharp(letter, modifier),
            eScaleDegree::SixFlat => Self::new_six_flat(letter, modifier),
            eScaleDegree::Six => Self::new_six(letter, modifier),
            eScaleDegree::SevenFlat => Self::new_seven_flat(letter, modifier),
            eScaleDegree::Seven => Self::new_seven(letter, modifier),
            eScaleDegree::Eight => Self::new_eight(letter, modifier),
            eScaleDegree::NineFlat => Self::new_nine_flat(letter, modifier),
            eScaleDegree::Nine => Self::new_nine(letter, modifier),
            eScaleDegree::NineSharp => Self::new_nine_sharp(letter, modifier),
            eScaleDegree::Eleven => Self::new_eleven(letter, modifier),
            eScaleDegree::ElevenSharp => Self::new_eleven_sharp(letter, modifier),
            eScaleDegree::ThirteenFlat => Self::new_thirteen_flat(letter, modifier),
            eScaleDegree::Thirteen => Self::new_thirteen(letter, modifier),
        }
    }

    pub fn get_letter(&self) -> eNoteLetter {
        self.letter
    }

    pub fn get_modifier(&self) -> eNoteModifier {
        self.modifier
    }

    pub fn get_scale_degree(&self) -> eScaleDegree {
        self.scale_degree
    }

    fn new_one(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::One;

        match (letter, modifier) {
            // F
              (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_two_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::TwoFlat;

        match (letter, modifier) {
            // G
              (eNoteLetter::G, eNoteModifier::DoubleFlat)
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            // D
            | (eNoteLetter::D, eNoteModifier::DoubleFlat)
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            // A
            | (eNoteLetter::A, eNoteModifier::DoubleFlat)
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            // E
            | (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_two(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Two;
        
        match (letter, modifier) {
            // G
              (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_three_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::ThreeFlat;
        
        match (letter, modifier) {
            // A
              (eNoteLetter::A, eNoteModifier::DoubleFlat)
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            // E
            | (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_three(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Three;
        
        match (letter, modifier) {
            // A
              (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            | (eNoteLetter::D, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_four(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Four;
        
        match (letter, modifier) {
            // B
              (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_four_sharp(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::FourSharp;
        
        match (letter, modifier) {
            // B
              (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            | (eNoteLetter::D, eNoteModifier::DoubleSharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            | (eNoteLetter::A, eNoteModifier::DoubleSharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            | (eNoteLetter::E, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_five_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::FiveFlat;
        
        match (letter, modifier) {
            // C
              (eNoteLetter::C, eNoteModifier::DoubleFlat)
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            // G
            | (eNoteLetter::G, eNoteModifier::DoubleFlat)
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            // D
            | (eNoteLetter::D, eNoteModifier::DoubleFlat)
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            // A
            | (eNoteLetter::A, eNoteModifier::DoubleFlat)
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            // E
            | (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_five(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Five;
        
        match (letter, modifier) {
            // C
              (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_five_sharp(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::FiveSharp;
        
        match (letter, modifier) {
            // C
              (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            | (eNoteLetter::D, eNoteModifier::DoubleSharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            | (eNoteLetter::A, eNoteModifier::DoubleSharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            | (eNoteLetter::E, eNoteModifier::DoubleSharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            | (eNoteLetter::B, eNoteModifier::DoubleSharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            | (eNoteLetter::F, eNoteModifier::TripleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_six_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::SixFlat;
        
        match (letter, modifier) {
            // D
              (eNoteLetter::D, eNoteModifier::DoubleFlat)
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            // A
            | (eNoteLetter::A, eNoteModifier::DoubleFlat)
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            // E
            | (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_six(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Six;
        
        match (letter, modifier) {
            // D
              (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_seven_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::SevenFlat;
        
        match (letter, modifier) {
            // E
              (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::DoubleFlat)
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_seven(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Seven;
        
        match (letter, modifier) {
            // E
              (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            | (eNoteLetter::D, eNoteModifier::DoubleSharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            | (eNoteLetter::A, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_eight(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Eight;
        
        match (letter, modifier) {
            // F
              (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_nine_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::NineFlat;
        
        match (letter, modifier) {
            // G
              (eNoteLetter::G, eNoteModifier::DoubleFlat)
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            // D
            | (eNoteLetter::D, eNoteModifier::DoubleFlat)
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            // A
            | (eNoteLetter::A, eNoteModifier::DoubleFlat)
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            // E
            | (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_nine(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Nine;
        
        match (letter, modifier) {
            // G
              (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_nine_sharp(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::NineSharp;
        
        match (letter, modifier) {
            // G
              (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            | (eNoteLetter::D, eNoteModifier::DoubleSharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            | (eNoteLetter::A, eNoteModifier::DoubleSharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            | (eNoteLetter::E, eNoteModifier::DoubleSharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            | (eNoteLetter::B, eNoteModifier::DoubleSharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            | (eNoteLetter::F, eNoteModifier::TripleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            | (eNoteLetter::C, eNoteModifier::TripleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_eleven(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Eleven;
        
        match (letter, modifier) {
            // B
              (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_eleven_sharp(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::ElevenSharp;
        
        match (letter, modifier) {
            // B
              (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp)
            // D
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            | (eNoteLetter::D, eNoteModifier::DoubleSharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            | (eNoteLetter::A, eNoteModifier::DoubleSharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            | (eNoteLetter::E, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_thirteen_flat(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::ThirteenFlat;
        
        match (letter, modifier) {
            // D
              (eNoteLetter::D, eNoteModifier::DoubleFlat)
            | (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            // A
            | (eNoteLetter::A, eNoteModifier::DoubleFlat)
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            // E
            | (eNoteLetter::E, eNoteModifier::DoubleFlat)
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            // B
            | (eNoteLetter::B, eNoteModifier::DoubleFlat)
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            // F
            | (eNoteLetter::F, eNoteModifier::Flat)
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Flat)
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Flat)
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }

    fn new_thirteen(letter: eNoteLetter, modifier: eNoteModifier) -> Result<Self, Box<dyn tError>> {
        let scale_degree = eScaleDegree::Thirteen;
        
        match (letter, modifier) {
            // D
              (eNoteLetter::D, eNoteModifier::Flat)
            | (eNoteLetter::D, eNoteModifier::Natural)
            | (eNoteLetter::D, eNoteModifier::Sharp)
            // A
            | (eNoteLetter::A, eNoteModifier::Flat)
            | (eNoteLetter::A, eNoteModifier::Natural)
            | (eNoteLetter::A, eNoteModifier::Sharp)
            // E
            | (eNoteLetter::E, eNoteModifier::Flat)
            | (eNoteLetter::E, eNoteModifier::Natural)
            | (eNoteLetter::E, eNoteModifier::Sharp)
            // B
            | (eNoteLetter::B, eNoteModifier::Flat)
            | (eNoteLetter::B, eNoteModifier::Natural)
            | (eNoteLetter::B, eNoteModifier::Sharp)
            // F
            | (eNoteLetter::F, eNoteModifier::Natural)
            | (eNoteLetter::F, eNoteModifier::Sharp)
            | (eNoteLetter::F, eNoteModifier::DoubleSharp)
            // C
            | (eNoteLetter::C, eNoteModifier::Natural)
            | (eNoteLetter::C, eNoteModifier::Sharp)
            | (eNoteLetter::C, eNoteModifier::DoubleSharp)
            // G
            | (eNoteLetter::G, eNoteModifier::Natural)
            | (eNoteLetter::G, eNoteModifier::Sharp)
            | (eNoteLetter::G, eNoteModifier::DoubleSharp) => Ok(Self { letter, modifier, scale_degree }),
            (letter, modifier) => Err(Box::new(sImpossibleOperationError::new(format!("'{}{}' is not a valid {} note", letter, modifier, scale_degree).as_str()))),
        }
    }
}

impl Display for sNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_letter(), self.get_letter())
    }
}

impl tKeyboardFriendlyName for sNote {
    fn get_keyboard_friendly_name(&self) -> String {
        format!("{}{}", self.letter.get_keyboard_friendly_name(), self.modifier.get_keyboard_friendly_name())
    }
}
