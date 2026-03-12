use crate::ast::sListValue;
use crate::ast::sObjectValue;
use crate::ast::sStringValue;
use crate::ast::tItem;
use crate::ast::tValue;

pub enum eValue {
    String(sStringValue),
    List(sListValue),
    Object(sObjectValue),
}

impl tItem for eValue {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String {
        match self {
            Self::String(string_value) => string_value.as_string(tabs, indent_first_line),
            Self::List(list_value) => list_value.as_string(tabs, indent_first_line),
            Self::Object(object_value) => object_value.as_string(tabs, indent_first_line),
        }
    }
}

impl tValue for eValue {
    fn has_same_composition(&self, other: &Self) -> bool {
        match self {
            Self::String(self_string_value) => {
                if let Self::String(other_string_value) = other {
                    return self_string_value.has_same_composition(other_string_value);
                }

                return false;
            }
            Self::List(self_list_value) => {
                if let Self::List(other_list_value) = other {
                    return self_list_value.has_same_composition(other_list_value);
                }

                return false;
            },
            Self::Object(self_object_value) => {
                if let Self::Object(other_object_value) = other {
                    return self_object_value.has_same_composition(other_object_value);
                }

                return false;
            },
        }
    }
}
