use crate::ast::eValue;
use crate::ast::tItem;
use crate::ast::tValue;

pub struct sListValue {
    values: Vec<eValue>,
}

impl sListValue {
    pub fn new(values: Vec<eValue>) -> Self {
        Self { values }
    }

    pub fn get_values(&self) -> &Vec<eValue> {
        &self.values
    }

    pub fn add_value(&mut self, value: eValue, line_index: usize, char_index: usize) {
        if self.values.len() == 0 {
            self.values.push(value);
            return;
        }

        let first_value = &self.values[0];

        if !first_value.has_same_composition(&value) {
            panic!("InterLang Parse Error @[line: {} | char: {}] :: Composition of list item doesn't match the composition of the first, which it must.", line_index, char_index);
        }

        self.values.push(value);
    }
}

impl tItem for sListValue {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        let mut as_string = "".to_string();

        let mut tabs_string = "".to_string();

        for _ in 0..tabs {
            tabs_string += "\t";
        }

        if indent_first_line {
            as_string += tabs_string.as_str();
        }

        as_string += "List(\n";

        let mut tabs = tabs;

        {
            tabs += 1;

            for value in self.get_values() {
                as_string += value.as_string(tabs, true).as_str();
                as_string += "\n";
            }
        }

        as_string += tabs_string.as_str();
        as_string += ")";

        as_string
    }
}

impl tValue for sListValue {
    fn has_same_composition(&self, other: &Self) -> bool {
        if self.values.len() > 0 && other.get_values().len() > 0 {
            let first_self_item = &self.values[0];
            let first_other_item = &other.get_values()[0];

            return first_self_item.has_same_composition(first_other_item);
        }

        return true;
    }
}
