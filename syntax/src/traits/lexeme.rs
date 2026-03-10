use std::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;

pub trait tLexeme : Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash + Debug + Display + Default {}
