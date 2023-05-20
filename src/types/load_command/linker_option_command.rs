use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `linker_option_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcLinkerOption {
    pub count: u32,
    // TODO: concatenation of zero terminated UTF8 strings.
    // Zero filled at end to align
    // strings: (),
}