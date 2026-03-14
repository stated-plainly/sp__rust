use crate::ast::sListValue;
use crate::ast::sObjectValue;
use crate::ast::sStringValue;
use crate::ast::tASTItem;

pub enum eValue {
    String(sStringValue),
    List(sListValue),
    Object(sObjectValue),
}

impl tASTItem for eValue {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        match self {
            Self::String(string_value) => string_value.as_string(tabs, indent_first_line),
            Self::List(list_value) => list_value.as_string(tabs, indent_first_line),
            Self::Object(object_value) => object_value.as_string(tabs, indent_first_line),
        }
    }

    fn has_same_composition(&self, other: &Self) -> bool {
        match self {
            Self::String(self_string_value) => {
                let Self::String(other_string_value) = other else { return false };

                return self_string_value.has_same_composition(other_string_value);
            }
            Self::List(self_list_value) => {
                let Self::List(other_list_value) = other else { return false };

                return self_list_value.has_same_composition(other_list_value);
            },
            Self::Object(self_object_value) => {
                let Self::Object(other_object_value) = other else { return false };

                return self_object_value.has_same_composition(other_object_value);
            },
        }
    }
}
