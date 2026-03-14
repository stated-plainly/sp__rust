use crate::ast::eValue;
use crate::ast::sKVP;
use crate::ast::tASTItem;

pub struct sObjectValue {
    kvps: Vec<sKVP>,
}

impl sObjectValue {
    pub fn new(kvps: Vec<sKVP>) -> Self {
        Self {
            kvps,
        }
    }

    pub fn get(&self, key: &str) -> Option<&eValue> {
        Some(self.kvps.iter().find(|kvp| kvp.get_key().get_key() == key)?.get_value())
    }
}

impl tASTItem for sObjectValue {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        let mut as_string = "".to_string();

        let mut tabs_string = "".to_string();

        for _ in 0..tabs {
            tabs_string += "\t";
        }

        if indent_first_line {
            as_string += tabs_string.as_str();
        }

        as_string += "Object(\n";

        let mut tabs = tabs;

        {
            tabs += 1;

            for kvp in &self.kvps {
                as_string += kvp.as_string(tabs, true).as_str();
                as_string += "\n";
            }

        }

        as_string += tabs_string.as_str();
        as_string += ")";

        as_string
    }

    fn has_same_composition(&self, other: &Self) -> bool {
        for self_kvp in &self.kvps {
            for other_kvp in &other.kvps {
                if !self_kvp.has_same_composition(other_kvp) {
                    return false;
                }
            }
        }

        return true;
    }
}
