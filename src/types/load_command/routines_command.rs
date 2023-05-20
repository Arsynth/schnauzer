use scroll::{IOread, SizeWith};

use std::fmt::Debug;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `routines_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcRoutines {
    pub init_address: u32,
    pub init_module: u32,

    /*
    uint32_t	reserved1;
    uint32_t	reserved2;
    uint32_t	reserved3;
    uint32_t	reserved4;
    uint32_t	reserved5;
    uint32_t	reserved6;
    */
    pub reserved: [u32; 6],
}

/// `routines_command_64`
#[repr(C)]
#[derive(Debug, IOread, SizeWith, AutoEnumFields)]
pub struct LcRoutines64 {
    pub init_address: u32,
    pub init_module: u32,

    /*
    uint64_t	reserved1;
    uint64_t	reserved2;
    uint64_t	reserved3;
    uint64_t	reserved4;
    uint64_t	reserved5;
    uint64_t	reserved6;
    */
    pub reserved: [u64; 6],
}