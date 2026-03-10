use identifier::traits::tIdentifier;

use crate::structs::sToken;

pub trait tLexeme<I: tIdentifier> {
    fn try_tokenise(&self, source_code: &str, char_index: usize) -> Option<sToken<I>>;
}
