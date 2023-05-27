//! Module provides Rust representations of `cpu_type_t` and `cpu_subtype_t`,
//! declared in `mach/machine.h` (<https://opensource.apple.com/source/xnu/xnu-6153.11.26/osfmk/mach/machine.h.auto.html>).
//! All constants listed in [cpu_constants] are taken from `mach/machine.h`.
//! Any skipped (commented) constants are not declared there too

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::BitOr;

use scroll::*;

use self::cpu_constants::*;
use crate::Hu32;

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

    /// Capability bits used in the definition of cpu_subtype.
    pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;
    pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000;

    ///	Object files that are hand-crafted to run on any
    ///	implementation of an architecture are tagged with
    ///	CPU_SUBTYPE_MULTIPLE.  This functions essentially the same as
    ///	the "ALL" subtype of an architecture except that it allows us
    ///	to easily find object files that may need to be modified
    ///	whenever a new implementation of an architecture comes out.
    ///
    ///	It is the responsibility of the implementor to make sure the
    ///	software handles unsupported implementations elegantly.
    pub const CPU_SUBTYPE_MULTIPLE: CPUSubtype = CPUSubtype(-1i32 as u32);
    pub const CPU_SUBTYPE_LITTLE_ENDIAN: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_BIG_ENDIAN: CPUSubtype = CPUSubtype(1);

    /// VAX subtypes (these do *not* necessary conform to the actual cpu
    /// ID assigned by DEC available via the SID register).
    pub const CPU_SUBTYPE_VAX_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_VAX780: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_VAX785: CPUSubtype = CPUSubtype(2);
    pub const CPU_SUBTYPE_VAX750: CPUSubtype = CPUSubtype(3);
    pub const CPU_SUBTYPE_VAX730: CPUSubtype = CPUSubtype(4);
    pub const CPU_SUBTYPE_UVAXI: CPUSubtype = CPUSubtype(5);
    pub const CPU_SUBTYPE_UVAXII: CPUSubtype = CPUSubtype(6);
    pub const CPU_SUBTYPE_VAX8200: CPUSubtype = CPUSubtype(7);
    pub const CPU_SUBTYPE_VAX8500: CPUSubtype = CPUSubtype(8);
    pub const CPU_SUBTYPE_VAX8600: CPUSubtype = CPUSubtype(9);
    pub const CPU_SUBTYPE_VAX8650: CPUSubtype = CPUSubtype(10);
    pub const CPU_SUBTYPE_VAX8800: CPUSubtype = CPUSubtype(11);
    pub const CPU_SUBTYPE_UVAXIII: CPUSubtype = CPUSubtype(12);

    /// 680x0 subtypes
    ///
    /// The subtype definitions here are unusual for historical reasons.
    /// NeXT used to consider 68030 code as generic 68000 code.  For
    /// backwards compatability:
    ///
    ///	CPU_SUBTYPE_MC68030 symbol has been preserved for source code
    ///	compatability.
    ///
    ///	CPU_SUBTYPE_MC680x0_ALL has been defined to be the same
    ///	subtype as CPU_SUBTYPE_MC68030 for binary comatability.
    ///
    ///	CPU_SUBTYPE_MC68030_ONLY has been added to allow new object
    ///	files to be tagged as containing 68030-specific instructions.

    pub const CPU_SUBTYPE_MC680x0_ALL: CPUSubtype = CPUSubtype(1);
    /// Same as [CPU_SUBTYPE_MC680x0_ALL] for compatibility
    pub const CPU_SUBTYPE_MC68030: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_MC68040: CPUSubtype = CPUSubtype(2);
    pub const CPU_SUBTYPE_MC68030_ONLY: CPUSubtype = CPUSubtype(3);

    macro_rules! CPU_SUBTYPE_INTEL {
        ($f:expr, $m:expr) => {
            CPUSubtype($f + ($m << 4))
        };
    }

    pub const CPU_SUBTYPE_I386_ALL: CPUSubtype = CPU_SUBTYPE_INTEL!(3, 0);
    pub const CPU_SUBTYPE_386: CPUSubtype = CPU_SUBTYPE_INTEL!(3, 0);
    pub const CPU_SUBTYPE_486: CPUSubtype = CPU_SUBTYPE_INTEL!(4, 0);
    pub const CPU_SUBTYPE_486SX: CPUSubtype = CPU_SUBTYPE_INTEL!(4, 8); // 8 << 4 = 128
    pub const CPU_SUBTYPE_586: CPUSubtype = CPU_SUBTYPE_INTEL!(5, 0);
    pub const CPU_SUBTYPE_PENT: CPUSubtype = CPU_SUBTYPE_INTEL!(5, 0);
    pub const CPU_SUBTYPE_PENTPRO: CPUSubtype = CPU_SUBTYPE_INTEL!(6, 1);
    pub const CPU_SUBTYPE_PENTII_M3: CPUSubtype = CPU_SUBTYPE_INTEL!(6, 3);
    pub const CPU_SUBTYPE_PENTII_M5: CPUSubtype = CPU_SUBTYPE_INTEL!(6, 5);
    pub const CPU_SUBTYPE_CELERON: CPUSubtype = CPU_SUBTYPE_INTEL!(7, 6);
    pub const CPU_SUBTYPE_CELERON_MOBILE: CPUSubtype = CPU_SUBTYPE_INTEL!(7, 7);
    pub const CPU_SUBTYPE_PENTIUM_3: CPUSubtype = CPU_SUBTYPE_INTEL!(8, 0);
    pub const CPU_SUBTYPE_PENTIUM_3_M: CPUSubtype = CPU_SUBTYPE_INTEL!(8, 1);
    pub const CPU_SUBTYPE_PENTIUM_3_XEON: CPUSubtype = CPU_SUBTYPE_INTEL!(8, 2);
    pub const CPU_SUBTYPE_PENTIUM_M: CPUSubtype = CPU_SUBTYPE_INTEL!(9, 0);
    pub const CPU_SUBTYPE_PENTIUM_4: CPUSubtype = CPU_SUBTYPE_INTEL!(10, 0);
    pub const CPU_SUBTYPE_PENTIUM_4_M: CPUSubtype = CPU_SUBTYPE_INTEL!(10, 1);
    pub const CPU_SUBTYPE_ITANIUM: CPUSubtype = CPU_SUBTYPE_INTEL!(11, 0);
    pub const CPU_SUBTYPE_ITANIUM_2: CPUSubtype = CPU_SUBTYPE_INTEL!(11, 1);
    pub const CPU_SUBTYPE_XEON: CPUSubtype = CPU_SUBTYPE_INTEL!(12, 0);
    pub const CPU_SUBTYPE_XEON_MP: CPUSubtype = CPU_SUBTYPE_INTEL!(12, 1);

    #[inline]
    pub const fn cpu_subtype_intel_family(x: u32) -> u32 {
        x & 15
    }
    pub const CPU_SUBTYPE_INTEL_FAMILY_MAX: u32 = 15;

    #[inline]
    pub const fn cpu_subtype_intel_model(x: u32) -> u32 {
        x >> 4
    }
    pub const CPU_SUBTYPE_INTEL_MODEL_ALL: u32 = 0;

    /// X86 subtypes.
    pub const CPU_SUBTYPE_X86_ALL: CPUSubtype = CPUSubtype(3);
    pub const CPU_SUBTYPE_X86_64_ALL: CPUSubtype = CPUSubtype(3);
    pub const CPU_SUBTYPE_X86_ARCH1: CPUSubtype = CPUSubtype(4);
    /// Haswell feature subset.
    pub const CPU_SUBTYPE_X86_64_H: CPUSubtype = CPUSubtype(8);

    pub const CPU_SUBTYPE_MIPS_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_MIPS_R2300: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_MIPS_R2600: CPUSubtype = CPUSubtype(2);
    pub const CPU_SUBTYPE_MIPS_R2800: CPUSubtype = CPUSubtype(3);
    /// pmax
    pub const CPU_SUBTYPE_MIPS_R2000A: CPUSubtype = CPUSubtype(4);
    pub const CPU_SUBTYPE_MIPS_R2000: CPUSubtype = CPUSubtype(5);
    /// 3max
    pub const CPU_SUBTYPE_MIPS_R3000A: CPUSubtype = CPUSubtype(6);
    pub const CPU_SUBTYPE_MIPS_R3000: CPUSubtype = CPUSubtype(7);

    /// MC98000 (PowerPC) subtypes
    pub const CPU_SUBTYPE_MC98000_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_MC98601: CPUSubtype = CPUSubtype(1);

    ///	HPPA subtypes for Hewlett-Packard HP-PA family of
    ///	risc processors. Port by NeXT to 700 series.

    pub const CPU_SUBTYPE_HPPA_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_HPPA_7100LC: CPUSubtype = CPUSubtype(1);

    ///	MC88000 subtypes.
    pub const CPU_SUBTYPE_MC88000_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_MC88100: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_MC88110: CPUSubtype = CPUSubtype(2);

    ///	SPARC subtypes
    pub const CPU_SUBTYPE_SPARC_ALL: CPUSubtype = CPUSubtype(0);

    /// I860 subtypes
    pub const CPU_SUBTYPE_I860_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_I860_860: CPUSubtype = CPUSubtype(1);

    ///	PowerPC subtypes
    pub const CPU_SUBTYPE_POWERPC_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_POWERPC_601: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_POWERPC_602: CPUSubtype = CPUSubtype(2);
    pub const CPU_SUBTYPE_POWERPC_603: CPUSubtype = CPUSubtype(3);
    pub const CPU_SUBTYPE_POWERPC_603E: CPUSubtype = CPUSubtype(4);
    pub const CPU_SUBTYPE_POWERPC_603EV: CPUSubtype = CPUSubtype(5);
    pub const CPU_SUBTYPE_POWERPC_604: CPUSubtype = CPUSubtype(6);
    pub const CPU_SUBTYPE_POWERPC_604E: CPUSubtype = CPUSubtype(7);
    pub const CPU_SUBTYPE_POWERPC_620: CPUSubtype = CPUSubtype(8);
    pub const CPU_SUBTYPE_POWERPC_750: CPUSubtype = CPUSubtype(9);
    pub const CPU_SUBTYPE_POWERPC_7400: CPUSubtype = CPUSubtype(10);
    pub const CPU_SUBTYPE_POWERPC_7450: CPUSubtype = CPUSubtype(11);
    pub const CPU_SUBTYPE_POWERPC_970: CPUSubtype = CPUSubtype(100);

    /// ARM subtypes
    pub const CPU_SUBTYPE_ARM_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_ARM_V4T: CPUSubtype = CPUSubtype(5);
    pub const CPU_SUBTYPE_ARM_V6: CPUSubtype = CPUSubtype(6);
    pub const CPU_SUBTYPE_ARM_V5TEJ: CPUSubtype = CPUSubtype(7);
    pub const CPU_SUBTYPE_ARM_XSCALE: CPUSubtype = CPUSubtype(8);
    /// ARMv7-A and ARMv7-R
    pub const CPU_SUBTYPE_ARM_V7: CPUSubtype = CPUSubtype(9);
    /// Cortex A9
    pub const CPU_SUBTYPE_ARM_V7F: CPUSubtype = CPUSubtype(10);
    /// Swift
    pub const CPU_SUBTYPE_ARM_V7S: CPUSubtype = CPUSubtype(11);
    pub const CPU_SUBTYPE_ARM_V7K: CPUSubtype = CPUSubtype(12);
    pub const CPU_SUBTYPE_ARM_V8: CPUSubtype = CPUSubtype(13);
    /// Not meant to be run under xnu
    pub const CPU_SUBTYPE_ARM_V6M: CPUSubtype = CPUSubtype(14);
    /// Not meant to be run under xnu
    pub const CPU_SUBTYPE_ARM_V7M: CPUSubtype = CPUSubtype(15);
    /// Not meant to be run under xnu
    pub const CPU_SUBTYPE_ARM_V7EM: CPUSubtype = CPUSubtype(16);
    /// Not meant to be run under xnu
    pub const CPU_SUBTYPE_ARM_V8M: CPUSubtype = CPUSubtype(17);

    /// ARM64 subtypes
    pub const CPU_SUBTYPE_ARM64_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_ARM64_V8: CPUSubtype = CPUSubtype(1);
    pub const CPU_SUBTYPE_ARM64E: CPUSubtype = CPUSubtype(2);

    ///  ARM64_32 subtypes
    pub const CPU_SUBTYPE_ARM64_32_ALL: CPUSubtype = CPUSubtype(0);
    pub const CPU_SUBTYPE_ARM64_32_V8: CPUSubtype = CPUSubtype(1);
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

    pub fn feature_flags(&self) -> Hu32 {
        Hu32((self.0 & CPU_SUBTYPE_MASK) >> 24)
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
