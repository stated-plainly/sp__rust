use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::ast::eValue;
use crate::ast::sKVP;
use crate::ast::tASTItem;

pub struct sInterLangObject {
    kvps: Vec<sKVP>,
}

impl sInterLangObject {
    pub fn new(kvps: Vec<sKVP>) -> Self {
        Self { kvps }
    }

    pub fn get(&self, key: &str) -> Option<&eValue> {
        Some(self.kvps.iter().find(|kvp| kvp.get_key().get_key() == key)?.get_value())
    }
}

impl Display for sInterLangObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut tabs: usize = 0;
        let mut as_string: String = "InterLangObject(\n".to_string();
        
        {
            tabs += 1;

            let mut tab_string: String = "".to_string();

            for _ in 0..tabs {
                tab_string += "\t";
            }

            for kvp in &self.kvps {
                as_string += tab_string.as_str();
                as_string += kvp.as_string(tabs, false).as_str();
            }
        }

        as_string += ")";

        write!(f, "{}", as_string)
    }
}
