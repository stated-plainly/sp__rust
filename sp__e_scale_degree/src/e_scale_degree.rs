use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum eScaleDegree {
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
