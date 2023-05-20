use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `thread_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcThread {
    flavor: u32,
    count: u32,
    /* struct XXX_thread_state state   thread state for this flavor */
    /* ... */
}