use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use crate::Version32;

/// `version_min_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcVersionMin {
    pub version: Version32,
    pub sdk: Version32,
}