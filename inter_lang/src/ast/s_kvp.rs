use crate::ast::eValue;
use crate::ast::sKey;
use crate::ast::tASTItem;

pub struct sKVP {
    key: sKey,
    value: eValue,
}

impl sKVP {
    pub fn new(key: sKey, value: eValue) -> Self {
        Self { key, value }
    }

    pub fn get_key(&self) -> &sKey {
        &self.key
    }

    pub fn get_value(&self) -> &eValue {
        &self.value
    }
}

impl tASTItem for sKVP {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        let mut as_string = "".to_string();

        let mut tab_string = "".to_string();

        for _ in 0..tabs {
            tab_string += "\t";
        }

        if indent_first_line {
            as_string += tab_string.as_str();
        }

        as_string += "KVP(\n";

        let mut tabs = tabs;

        {
            tabs += 1;
            tab_string += "\t";

            as_string += tab_string.as_str();
            as_string += format!("key: {}\n", self.get_key().as_string(tabs, false)).as_str();
            as_string += tab_string.as_str();
            as_string += format!("value: {}\n", self.get_value().as_string(tabs, false)).as_str();

            tab_string.pop();
        }

        as_string += tab_string.as_str();
        as_string += ")";

        as_string
    }

    fn has_same_composition(&self, other: &Self) -> bool {
        if !self.get_key().has_same_composition(other.get_key()) {
            return false;
        }

        if !self.get_value().has_same_composition(other.get_value()) {
            return false;
        }

        true
    }
}
