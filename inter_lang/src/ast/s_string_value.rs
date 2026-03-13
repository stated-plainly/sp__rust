use crate::ast::tASTItem;

pub struct sStringValue {
    value: String,
}

impl sStringValue {
    pub fn new(value: &str) -> Self {
        Self { value: value.to_string() }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl tASTItem for sStringValue {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        let mut as_string: String = "".to_string();

        if indent_first_line {
            for _ in 0..tabs {
                as_string += "\t";
            }
        }

        as_string += format!("String({})", self.get_value()).as_str();

        as_string
    }

    fn has_same_composition(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
