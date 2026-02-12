use std::fmt::Display;

use sp__t_keyboard_friendly_name::tKeyboardFriendlyName;

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

impl Display for eScaleDegree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "{}", "1"),
            Self::TwoFlat => write!(f, "{}", "2♭"),
            Self::Two => write!(f, "{}", "2"),
            Self::ThreeFlat => write!(f, "{}", "3♭"),
            Self::Three => write!(f, "{}", "3"),
            Self::Four => write!(f, "{}", "4"),
            Self::FourSharp => write!(f, "{}", "4♯"),
            Self::FiveFlat => write!(f, "{}", "5♭"),
            Self::Five => write!(f, "{}", "5"),
            Self::FiveSharp => write!(f, "{}", "5♯"),
            Self::SixFlat => write!(f, "{}", "6♭"),
            Self::Six => write!(f, "{}", "6"),
            Self::SevenFlat => write!(f, "{}", "7♭"),
            Self::Seven => write!(f, "{}", "7"),
            Self::Eight => write!(f, "{}", "8"),
            Self::NineFlat => write!(f, "{}", "9♭"),
            Self::Nine => write!(f, "{}", "9"),
            Self::NineSharp => write!(f, "{}", "9♯"),
            Self::Eleven => write!(f, "{}", "11"),
            Self::ElevenSharp => write!(f, "{}", "11♯"),
            Self::ThirteenFlat => write!(f, "{}", "13♭"),
            Self::Thirteen => write!(f, "{}", "13"),
        }
    }
}

impl tKeyboardFriendlyName for eScaleDegree {
    fn get_keyboard_friendly_name(&self) -> String {
        match self {
            Self::One => "1",
            Self::TwoFlat => "2b",
            Self::Two => "2",
            Self::ThreeFlat => "3b",
            Self::Three => "3",
            Self::Four => "4",
            Self::FourSharp => "4s",
            Self::FiveFlat => "5b",
            Self::Five => "5",
            Self::FiveSharp => "5s",
            Self::SixFlat => "6b",
            Self::Six => "6",
            Self::SevenFlat => "7b",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::NineFlat => "9b",
            Self::Nine => "9",
            Self::NineSharp => "9s",
            Self::Eleven => "11",
            Self::ElevenSharp => "11s",
            Self::ThirteenFlat => "13b",
            Self::Thirteen => "13",
        }.to_string()
    }
}
