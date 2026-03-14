use std::fmt::Debug;
use std::fmt::Display;

pub trait tTokenIdentifier : Copy + Clone + PartialEq + Eq + Debug + Display {
    fn as_vec() -> Vec<Self>;
}
