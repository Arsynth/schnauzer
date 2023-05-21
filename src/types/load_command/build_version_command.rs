use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::{auto_enum_fields::*, Version32};
use schnauzer_derive::AutoEnumFields;

/// `build_version_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcBuildVersion {
    pub platform: u32,
    pub minos: Version32,
    pub sdk: Version32,
    pub ntools: u32,
    // TODO: Accurate way to provide BuildToolVersion
    // tools: (),
}