/// <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/nlist.h.auto.html>


use scroll::SizeWith;
use scroll::{IOread};

use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

#[derive(AutoEnumFields)]
pub enum NlistVariant {
    Nlist32(Nlist32),
    Nlist64(Nlist64),
}

impl NlistVariant {
    pub fn is_64(&self) -> bool {
        match self {
            NlistVariant::Nlist32(_) => false,
            NlistVariant::Nlist64(_) => true,
        }
    }
}

#[repr(C)]
#[derive(IOread, SizeWith, AutoEnumFields)]
pub struct Nlist32 {
    /// In the original `nlist` struct this field is uniun - `n_un`
    pub n_strx: u32,
    pub n_type: u8,
	pub n_sect: u8,
	pub n_desc: u16,
	pub n_value: u32,
}

#[repr(C)]
#[derive(IOread, SizeWith, AutoEnumFields)]
pub struct Nlist64 {
    pub n_strx: u32,
    pub n_type: u8,
	pub n_sect: u8,
	pub n_desc: u16,
	pub n_value: u64,
}