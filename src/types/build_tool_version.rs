use scroll::{IOread, SizeWith};

use std::fmt::{Debug};

use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `build_tool_version`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct BuildToolVersion {
    pub tool: u32,
    pub version: u32,
}