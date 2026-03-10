use crate::ast::sListKVP;
use crate::ast::sObjectKVP;
use crate::ast::sStringKVP;

pub enum eKVP {
    String(sStringKVP),
    List(sListKVP),
    Object(sObjectKVP),
}
