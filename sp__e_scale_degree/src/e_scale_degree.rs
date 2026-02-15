use std::fmt::Display;

use sp__s_impossible_operation_error::sImpossibleOperationError;
use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

const DISPLAY_NAMES: &[&str] = &[
    "1",
    "2♭",
    "2",
    "3♭",
    "3",
    "4",
    "4♯",
    "5♭",
    "5",
    "5♯",
    "6♭",
    "6",
    "7♭",
    "7",
    "8",
    "9♭",
    "9",
    "9♯",
    "11",
    "11♯",
    "13♭",
    "13",
];

const KEYBOARD_FRIENDLY_NAMES: &[&str] = &[
    "1",
    "2b",
    "2",
    "3b",
    "3",
    "4",
    "4s",
    "5b",
    "5",
    "5s",
    "6b",
    "6",
    "7b",
    "7",
    "8",
    "9b",
    "9",
    "9s",
    "11",
    "11s",
    "13b",
    "13",
];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum eScaleDegree {
    #[default]
    One,
    TwoFlat,
    Two,
    ThreeFlat,
    Three,
    Four,
    FourSharp,
    FiveFlat,
    Five,
    FiveSharp,
    SixFlat,
    Six,
    SevenFlat,
    Seven,
    Eight,
    NineFlat,
    Nine,
    NineSharp,
    Eleven,
    ElevenSharp,
    ThirteenFlat,
    Thirteen,
}

impl TryFrom<usize> for eScaleDegree {
    type Error = sImpossibleOperationError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::One),
            1 => Ok(Self::TwoFlat),
            2 => Ok(Self::Two),
            3 => Ok(Self::ThreeFlat),
            4 => Ok(Self::Three),
            5 => Ok(Self::Four),
            6 => Ok(Self::FourSharp),
            7 => Ok(Self::FiveFlat),
            8 => Ok(Self::Five),
            9 => Ok(Self::FiveSharp),
            10 => Ok(Self::SixFlat),
            11 => Ok(Self::Six),
            12 => Ok(Self::SevenFlat),
            13 => Ok(Self::Seven),
            14 => Ok(Self::Eight),
            15 => Ok(Self::NineFlat),
            16 => Ok(Self::Nine),
            17 => Ok(Self::NineSharp),
            18 => Ok(Self::Eleven),
            19 => Ok(Self::ElevenSharp),
            20 => Ok(Self::ThirteenFlat),
            21 => Ok(Self::Thirteen),
            index => Err(sImpossibleOperationError::new(format!("'{}' is not a valid eScaleDegree index", index).as_str())),
        }
    }
}

impl Into<usize> for eScaleDegree {
    fn into(self) -> usize {
        let index: usize = self.try_into().expect("The conversion can only fail from the usize -> eScaleDegree side. As we're converting from the eScaleDegree, we know that the conversion will succeed.");

        index
    }
}

impl Display for eScaleDegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index: usize = self.clone().into();

        write!(f, "{}", DISPLAY_NAMES[index])
    }
}

impl TryFrom<String> for eScaleDegree {
    type Error = sImpossibleOperationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        for (index, keyboard_friendly_name) in KEYBOARD_FRIENDLY_NAMES.iter().enumerate() {
            if value == *keyboard_friendly_name {
                let scale_degree: Self = index.try_into().expect("The index is auto-generated from the associated KEYBOARD_FRIENDLY_NAMES array, guaranteeing that this conversion will succeed.");
                
                return Ok(scale_degree);
            }
        }

        Err(sImpossibleOperationError::new(format!("'{}' is not a valid String representation of a eScaleDegree", value).as_str()))
    }
}

impl tKeyboardFriendlyName for eScaleDegree {
    fn get_keyboard_friendly_name(&self) -> String {
        let index: usize = self.clone().into();

        KEYBOARD_FRIENDLY_NAMES[index].to_string()
    }
}
