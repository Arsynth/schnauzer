use std::fmt::*;

pub trait AutoEnumFields {

}

pub enum Field {
    String(String),
    U32(u32),
    /// Value and hex output width
    HexU32(u32, usize),
}

impl Debug for Field {
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