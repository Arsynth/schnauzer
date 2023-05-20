use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `dysymtab_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcDysimtab {
    pub ilocalsym: u32,
    pub nlocalsym: u32,

    pub iextdefsym: u32,
    pub nextdefsym: u32,

    pub iundefsym: u32,
    pub nundefsym: u32,

    pub tocoff: u32,
    pub ntoc: u32,

    pub modtaboff: u32,
    pub nmodtab: u32,

    pub extrefsymoff: u32,
    pub nextrefsyms: u32,

    pub indirectsymoff: u32,
    pub nindirectsyms: u32,

    pub extreloff: u32,
    pub nextrel: u32,

    pub locreloff: u32,
    pub nlocrel: u32,
}