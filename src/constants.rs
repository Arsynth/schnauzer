use scroll::SizeWith;
use scroll::{IOread};
use std::fmt::{LowerHex, Debug, Display};
use super::fmt_ext::*;

pub const BYTES_PER_MAGIC: usize = 4;
pub const BYTES_PER_FAT_HEADER: usize = 8;
pub const BYTES_PER_FAT_ARCH: usize = 20;
pub const BYTES_PER_LOAD_COMMAND: usize = 8;
pub const BYTES_PER_SECTION32: usize = 68;
pub const BYTES_PER_SECTION64: usize = 80;
pub const BYTES_PER_NLIST32: usize = 12;
pub const BYTES_PER_NLIST64: usize = 16;

/// Represents vm_prot_t
pub type VmProt = Hi32;

pub type LoadCommandType = u32;

/// <https://opensource.apple.com/source/xnu/xnu-4570.41.2/osfmk/mach/machine.h.auto.html>

pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;
pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000;

pub const CPU_ARCH_ABI64: u32 = 0x01000000;

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

impl Display for Hu32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Hu32w4(pub u32);

impl Debug for Hu32w4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl LowerHex for Hu32w4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

impl Display for Hu32w4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", self.0)
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

impl Display for Hi32 {
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

impl Display for Hu64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}

/// Represents cpu_type_t
#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct CPUType(pub u32);

impl CPUType {
    pub fn is_64(&self) -> bool {
        (self.0 & CPU_ARCH_ABI64) == CPU_ARCH_ABI64
    }
}

impl Debug for CPUType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for CPUType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents cpu_subtype_t
#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct CPUSubtype(pub u32);

impl CPUSubtype {
    pub fn masked(&self) -> u32 {
        self.0 & !CPU_SUBTYPE_MASK
    }

    pub fn feature_flags(&self, ) -> Hu32w4 {
        Hu32w4((self.0 & CPU_SUBTYPE_MASK) >> 24)
    }
}

impl Debug for CPUSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.masked())
    }
}

impl Display for CPUSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.masked())
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

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Version32(pub u32);

impl Version32 {
    pub fn x(&self) -> u32 {
        const MASK: u32 = 0xFFFF0000;
        (self.0 & MASK) >> 16
    }

    pub fn y(&self) -> u32 {
        const MASK: u32 = 0x0000FF00;
        (self.0 & MASK) >> 8
    }
    
    pub fn z(&self) -> u32 {
        const MASK: u32 = 0x000000FF;
        self.0 & MASK
    }
}

impl Debug for Version32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.x(), self.y(), self.z())
    }
}

impl Display for Version32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.x(), self.y(), self.z())
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Version64(pub u64);

impl Version64 {
    pub fn a(&self) -> u64 {
        const MASK: u64 = 0xFFFFFF0000000000;
        (self.0 & MASK) >> 40
    }
    pub fn b(&self) -> u64 {
        const MASK: u64 = 0xFFC0000000;
        (self.0 & MASK) >> 30
    }
    pub fn c(&self) -> u64 {
        const MASK: u64 = 0x3FF00000;
        (self.0 & MASK) >> 20
    }
    pub fn d(&self) -> u64 {
        const MASK: u64 = 0xFFC00;
        (self.0 & MASK) >> 10
    }
    pub fn e(&self) -> u64 {
        const MASK: u64 = 0x3FF;
        self.0 & MASK
    }
}

impl Debug for Version64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}.{}", self.a(), self.b(), self.c(), self.d(), self.e())
    }
}

impl Display for Version64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}.{}.{}", self.a(), self.b(), self.c(), self.d(), self.e())
    }
}

#[repr(transparent)]
#[derive(IOread, SizeWith)]
pub struct Str16Bytes(pub [u8; 16]);

impl Debug for Str16Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_string(&self.0))
    }
}

impl Display for Str16Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &printable_string(&self.0))
    }
}