use std::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;

pub trait tIdentifier : Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash + Debug + Display + Default {}
