use std::fmt::Debug;
use std::fmt::Display;
use std::ops::BitOr;

use scroll::*;

use self::cpu_constants::*;
use crate::Hu32w4;

/// All constants from `mach/machine.h` (<https://opensource.apple.com/source/xnu/xnu-6153.11.26/osfmk/mach/machine.h.auto.html>).
/// Any skipped (commented) constants are not declared there too
#[allow(non_upper_case_globals)]
pub mod cpu_constants {
    use super::CPUSubtype;
    use super::CPUType;

    /// Mask for architecture bits.
    pub const CPU_ARCH_MASK: u32 = 0xff000000;
    /// 64 bit ABI.
    pub const CPU_ARCH_ABI64: u32 = 0x01000000;
    /// ABI for 64-bit hardware with 32-bit types; LP32
    pub const CPU_ARCH_ABI64_32: u32 = 0x02000000;

    pub const CPU_TYPE_ANY: u32 = -1i32 as u32;

    pub const CPU_TYPE_VAX: CPUType = CPUType(1);
    pub const CPU_TYPE_MC680x0: CPUType = CPUType(6);
    pub const CPU_TYPE_X86: CPUType = CPUType(7);
    /// Same as [CPU_TYPE_X86] for compatibility
    pub const CPU_TYPE_I386: CPUType = CPU_TYPE_X86;
    pub const CPU_TYPE_X86_64: CPUType = CPUType(CPU_TYPE_X86.0 | CPU_ARCH_ABI64);

    pub const CPU_TYPE_MC98000: CPUType = CPUType(10);
    pub const CPU_TYPE_HPPA: CPUType = CPUType(11);
    pub const CPU_TYPE_ARM: CPUType = CPUType(12);
    pub const CPU_TYPE_ARM64: CPUType = CPUType(CPU_TYPE_ARM.0 | CPU_ARCH_ABI64);
    pub const CPU_TYPE_ARM64_32: CPUType = CPUType(CPU_TYPE_ARM.0 | CPU_ARCH_ABI64_32);
    pub const CPU_TYPE_MC88000: CPUType = CPUType(13);
    pub const CPU_TYPE_SPARC: CPUType = CPUType(14);
    pub const CPU_TYPE_I860: CPUType = CPUType(15);
    pub const CPU_TYPE_POWERPC: CPUType = CPUType(18);
    pub const CPU_TYPE_POWERPC64: CPUType = CPUType(CPU_TYPE_POWERPC.0 | CPU_ARCH_ABI64);

    pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;
    pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000;

    pub const CPU_SUBTYPE_X86_ALL: CPUSubtype = CPUSubtype(3);
    pub const CPU_SUBTYPE_X86_64_ALL: CPUSubtype = CPUSubtype(3);
    pub const CPU_SUBTYPE_X86_ARCH1: CPUSubtype = CPUSubtype(4);
    pub const CPU_SUBTYPE_X86_64_H: CPUSubtype = CPUSubtype(8);

    pub const CPU_SUBTYPE_ARM64_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_ARM64_V8: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_ARM64E: CPUSubtype = CPUSubtype(2);
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
        write!(f, "{}", self.masked().0)
    }
}

impl Display for CPUSubtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.masked().0)
    }
}

impl Copy for CPUSubtype {}
