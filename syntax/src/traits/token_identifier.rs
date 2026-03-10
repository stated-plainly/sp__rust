use std::hash::Hash;
use std::fmt::Debug;
use std::fmt::Display;

pub trait tTokenIdentifier : Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Hash + Debug + Display + Default {}
