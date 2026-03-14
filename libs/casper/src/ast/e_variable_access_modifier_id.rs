#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum eVariableAccessModifierID {
    Immut,
    Mut,
    MutIn,
    MutEx,
}
