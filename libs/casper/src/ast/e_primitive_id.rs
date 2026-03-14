use std::fmt::Display;

use identifier::traits::tIdentifier;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ePrimitiveID {
    // bit
    Bit,
    // b{size}
    B8,
    B16,
    B32,
    B64,
    B128,
    BSize,
    // h{size}
    H8,
    H16,
    H32,
    H64,
    H128,
    HSize,
    // u{size}
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    // i{size}
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    // f{size}
    F32,
    F64,
    // bool
    Bool,
    // char
    Char,
    // string
    String,
    // array
    Array,
    // list
    List,
    // tuple
    Tuple,
    // range
    Range,
    // slice
    Slice,
}

impl tIdentifier for ePrimitiveID {
    fn as_vec() -> Vec<Self> {
        vec![
            // bit
            Self::Bit,
            // b{size}
            Self::B8,
            Self::B16,
            Self::B32,
            Self::B64,
            Self::B128,
            Self::BSize,
            // h{size}
            Self::H8,
            Self::H16,
            Self::H32,
            Self::H64,
            Self::H128,
            Self::HSize,
            // u{size}
            Self::U8,
            Self::U16,
            Self::U32,
            Self::U64,
            Self::U128,
            Self::USize,
            // i{size}
            Self::I8,
            Self::I16,
            Self::I32,
            Self::I64,
            Self::I128,
            Self::ISize,
            // f{size}
            Self::F32,
            Self::F64,
            // bool
            Self::Bool,
            // char
            Self::Char,
            // string
            Self::String,
            // array
            Self::Array,
            // list
            Self::List,
            // tuple
            Self::Tuple,
            // range
            Self::Range,
            // slice
            Self::Slice,
        ]
    }
}

impl Display for ePrimitiveID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            // bit
            Self::Bit => "bit",
            // b{size}
            Self::B8 => "b8",
            Self::B16 => "b16",
            Self::B32 => "b32",
            Self::B64 => "b64",
            Self::B128 => "b128",
            Self::BSize => "bsize",
            // h{size}
            Self::H8 => "h8",
            Self::H16 => "h16",
            Self::H32 => "h32",
            Self::H64 => "h64",
            Self::H128 => "h128",
            Self::HSize => "hsize",
            // u{size}
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::U128 => "u128",
            Self::USize => "usize",
            // i{size}
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::I128 => "i128",
            Self::ISize => "isize",
            // f{size}
            Self::F32 => "f32",
            Self::F64 => "f64",
            // bool
            Self::Bool => "bool",
            // char
            Self::Char => "char",
            // string
            Self::String => "string",
            // array
            Self::Array => "array",
            // list
            Self::List => "list",
            // tuple
            Self::Tuple => "tuple",
            // range
            Self::Range => "range",
            // slice
            Self::Slice => "slice",
        };

        write!(f, "{}", as_str)
    }
}
