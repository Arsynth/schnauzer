use scroll::SizeWith;
use scroll::{IOread};
use std::fmt::{LowerHex, Debug};
use super::fmt_ext::*;

pub const BYTES_PER_MAGIC: usize = 4;
pub const BYTES_PER_FAT_HEADER: usize = 8;
pub const BYTES_PER_FAT_ARCH: usize = 20;
pub const BYTES_PER_LOAD_COMMAND: usize = 8;

/// Represents cpu_type_t
pub type CPUType = u32;
/// Represents cpu_subtype_t
pub type CPUSubtype = u32;
/// Represents vm_prot_t
pub type VmProt = Hi32;
/// Represents `union lc_str`
pub type LcStr = u32;

pub type LoadCommandType = u32;

/// <https://opensource.apple.com/source/xnu/xnu-4570.41.2/osfmk/mach/machine.h.auto.html>

pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;
pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000;

pub const CPU_ARCH_ABI64: CPUType = 0x01000000;

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu32(pub u32);

impl Debug for Hu32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl LowerHex for Hu32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hi32(pub i32);

impl Debug for Hi32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl LowerHex for Hi32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu64(pub u64);

impl Debug for Hu64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}

impl LowerHex for Hu64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Uuid(pub [u8; 16]);

impl Debug for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_uuid_string(&self.0))
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Segname(pub [u8; 16]);

impl Debug for Segname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_string(&self.0))
    }
}