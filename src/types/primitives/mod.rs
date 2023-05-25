use crate::fmt_ext::printable_string;
use crate::fmt_ext::printable_uuid_string;
use scroll::{IOread, SizeWith};
use std::fmt::{Debug, Display, LowerHex};
use std::ops::BitOr;

use super::auto_enum_fields::*;
use super::constants::*;

pub mod filetype;
pub use filetype::*;

pub mod object_flags;
pub use object_flags::*;

pub mod machine;
pub use machine::*;

/// Represents vm_prot_t
pub type VmProt = Hi32;

pub type LoadCommandType = u32;

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

pub enum U64U32 {
    U32(u32),
    U64(u64),
}

impl U64U32 {
    pub fn hex_string(&self) -> String {
        match self {
            U64U32::U32(v) => Hu32(*v).to_string(),
            U64U32::U64(v) => Hu64(*v).to_string(),
        }
    }
}

impl AutoEnumFields for U64U32 {
    fn all_fields(&self) -> Vec<Field> {
        let field = match self {
            Self::U32(val) => Field::new("u32".to_string(), val.to_string()),
            Self::U64(val) => Field::new("u64".to_string(), val.to_string()),
        };

        vec![field]
    }
}

impl std::fmt::Debug for U64U32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U32(arg0) => f.debug_tuple("U32").field(arg0).finish(),
            Self::U64(arg0) => f.debug_tuple("U64").field(arg0).finish(),
        }
    }
}

impl Display for U64U32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            U64U32::U32(v) => write!(f, "{v}"),
            U64U32::U64(v) => write!(f, "{v}"),
        }
    }
}

/// Represents cpu_type_t
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, IOread, SizeWith)]
pub struct CPUType(pub u32);

impl CPUType {
    pub fn is_64(&self) -> bool {
        (self.0 & CPU_ARCH_ABI64) == CPU_ARCH_ABI64
    }
}

impl BitOr<u32> for CPUType {
    type Output = Self;

    fn bitor(self, rhs: u32) -> Self::Output {
        CPUType(self.0 | rhs)
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

impl Copy for CPUType {}

/// Represents cpu_subtype_t
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, IOread, SizeWith)]
pub struct CPUSubtype(pub u32);

impl CPUSubtype {
    pub fn masked(&self) -> CPUSubtype {
        CPUSubtype(self.0 & !CPU_SUBTYPE_MASK)
    }

    pub fn feature_flags(&self) -> Hu32w4 {
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

impl Copy for CPUSubtype {}

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
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.a(),
            self.b(),
            self.c(),
            self.d(),
            self.e()
        )
    }
}

impl Display for Version64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.a(),
            self.b(),
            self.c(),
            self.d(),
            self.e()
        )
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
