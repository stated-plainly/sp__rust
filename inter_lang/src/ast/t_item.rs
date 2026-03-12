pub trait tItem {
    fn as_string(&self, tabs: u8, indent_first_line: bool) -> String;
}
