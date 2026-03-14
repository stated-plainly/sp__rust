use std::fmt::Display;

use identifier::traits::tIdentifier;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum eVariableAccessModifierID {
    Immut,
    Mut,
    MutIn,
    MutEx,
}

impl tIdentifier for eVariableAccessModifierID {
    fn as_vec() -> Vec<Self> {
        vec![
            Self::Immut,
            Self::Mut,
            Self::MutIn,
            Self::MutEx,
        ]
    }
}

impl Display for eVariableAccessModifierID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Self::Immut => "{immut}",
            Self::Mut => "mut",
            Self::MutIn => "mut:in",
            Self::MutEx => "mut:ex",
        };

        write!(f, "{}", as_str)
    }
}
