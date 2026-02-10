use std::fmt::Display;

use sp__s_borrower::sBorrower;
use sp__t_token_kind::tTokenKind;

pub struct sToken<K: tTokenKind> {
    kind: K,
    value: sBorrower<String>,
    line_index: usize,
    char_index: usize,
}

impl<K: tTokenKind> sToken<K> {
    pub fn new(kind: K, value: sBorrower<String>, line_index: usize, char_index: usize) -> Self {
        Self {
            kind,
            value,
            line_index,
            char_index,
        }
    }

    pub fn get_kind(&self) -> K {
        self.kind
    }

    pub fn get_value(&self) -> sBorrower<String> {
        self.value.clone()
    }

    pub fn get_line_index(&self) -> usize {
        self.line_index
    }

    pub fn get_char_index(&self) -> usize {
        self.char_index
    }
}

impl<K: tTokenKind> Display for sToken<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token[{} :: \"{}\"]", self.kind, &*self.value)
    }
}
