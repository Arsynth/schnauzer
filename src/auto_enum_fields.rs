use std::fmt::*;
use schnauzer_derive::AutoEnumFields;

pub trait AutoEnumFields {
    fn all_fields(&self) -> Vec<Field>;
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
}

impl Field {
    pub fn new(name: String, value: FieldValue) -> Self {
        Field { name: name, value: value }
    } 
}

pub enum FieldValue {
    String(String),
    U32(u32),
    /// Value and hex output min width
    HexU32(u32, usize),
}

impl Debug for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::String(arg0) => write!(f, "{}", arg0),
            Self::U32(arg0) => write!(f, "{}", arg0.to_string()),
            Self::HexU32(arg0, width) => {
                let arg0 = format!("{:#0w$x}", arg0, w = width);
                write!(f, "{}", arg0) 
            },
        }
    }
}

#[derive(AutoEnumFields)]
pub struct MyHeader64 {
    pub ncmds: u32,
    pub size: u64,

    pub name: String
}
