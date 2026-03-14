pub trait tASTItem {
    fn as_string(&self, tabs: usize, indent_first_line: bool) -> String;
 
    fn has_same_composition(&self, other: &Self) -> bool;
}
