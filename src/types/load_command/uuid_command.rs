use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use crate::Uuid;

/// `uuid_command`
#[repr(C)]
#[derive(IOread, SizeWith, AutoEnumFields)]
pub struct LcUuid {
    pub uuid: Uuid,
}

impl Debug for LcUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcUuid").field("uuid", &self.uuid).finish()
    }
}