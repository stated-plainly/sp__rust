use crate::structs::sToken;
use crate::traits::tTokenIdentifier;

pub trait tLexeme<TI: tTokenIdentifier> {
    fn try_tokenise(&self, source_code: &str, char_index: usize) -> Option<sToken<TI>>;
}
