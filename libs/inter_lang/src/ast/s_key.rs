use crate::ast::tASTItem;

pub struct sKey {
    key: String,
}

impl sKey {
    pub fn new(key: &str) -> Self {
        Self { key: key.to_string() }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }
}

impl tASTItem for sKey {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        let mut as_string = "".to_string();

        if indent_first_line {
            for _ in 0..tabs {
                as_string += "\t";
            }
        }

        as_string += format!("Key({})", self.get_key()).as_str();

        as_string
    }

    fn has_same_composition(&self, other: &Self) -> bool {
        self.get_key() == other.get_key()
    }
}
