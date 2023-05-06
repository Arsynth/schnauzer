use std::fmt::*;

pub trait AutoEnumFields {
    fn all_fields(&self) -> Vec<Field>;
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: String,
}

impl Field {
    pub fn new(name: String, value: String) -> Self {
        Field { name: name, value: value }
    } 
}
