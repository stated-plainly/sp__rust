use crate::ast::eKVP;

pub struct sInterLangObject {
    kvps: Vec<eKVP>,
}

impl sInterLangObject {
    pub fn new() -> Self {
        Self {
            kvps: vec![],
        }
    }
}
