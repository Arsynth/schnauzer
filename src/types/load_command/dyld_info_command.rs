use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `dyld_info_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcDyldInfo {
    pub rebase_off: u32,
    pub rebase_size: u32,

    pub bind_off: u32,
    pub bind_size: u32,

    pub weak_bind_off: u32,
    pub weak_bind_size: u32,

    pub lazy_bind_off: u32,
    pub lazy_bind_size: u32,

    pub export_off: u32,
    pub export_size: u32,
}