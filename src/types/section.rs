use super::primitives::*;

use std::fmt::{Debug};
use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;
use scroll::SizeWith;
use scroll::{IOread};

#[derive(IOread, SizeWith, Debug, AutoEnumFields)]
pub struct Section32 {
    pub sectname: Str16Bytes,
    pub segname: Str16Bytes,
    pub addr: Hu32,
    pub size: Hu32,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: Hu32,
    pub reserved1: u32,
    pub reserved2: u32,
}


#[derive(IOread, SizeWith, Debug, AutoEnumFields)]
pub struct Section64 {
    pub sectname: Str16Bytes,
    pub segname: Str16Bytes,
    pub addr: Hu64,
    pub size: Hu64,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: Hu32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}