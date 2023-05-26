//! <https://opensource.apple.com/source/xnu/xnu-6153.11.26/osfmk/mach/machine.h.auto.html>

use std::fmt::{Debug, Display};

pub mod cpu;
pub use cpu::*;

use self::cpu_constants::*;

/// Convenience struct for printing architecture short string.
/// As this structure implements [std::fmt::Display], its function 'to_string()' returns values like: arm64, arm64e, x86_64, x86_64h.
/// TODO: Support other architectures.
#[allow(non_camel_case_types)]
pub enum PrintableCPU {
    x86_64(SubtypeX86_64),
    arm64(SubtypeArm64),
}

impl PrintableCPU {
    /// Returns `PrintableCPU` if both `cputype` and `cpusubtype` supported by printable structure.
    pub fn new(cputype: CPUType, cpusubtype: CPUSubtype) -> Option<Self> {
        match cputype {
            CPU_TYPE_X86_64 => match SubtypeX86_64::new(cpusubtype) {
                Some(s) => Some(Self::x86_64(s)),
                None => None,
            },
            CPU_TYPE_ARM64 => match SubtypeArm64::new(cpusubtype) {
                Some(s) => Some(Self::arm64(s)),
                None => None,
            },
            _ => None,
        }
    }
}

impl Debug for PrintableCPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::x86_64(arg0) => write!(f, "{}", arg0),
            Self::arm64(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl Display for PrintableCPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SubtypeX86_64 {
    x86_64,
    x86_64h,
}

impl SubtypeX86_64 {
    pub fn new(cpu_subtype: CPUSubtype) -> Option<Self> {
        match cpu_subtype.masked() {
            CPU_SUBTYPE_X86_64_ALL => Some(Self::x86_64),
            CPU_SUBTYPE_X86_64_H => Some(Self::x86_64h),
            _ => None,
        }
    }
}

impl Display for SubtypeX86_64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum SubtypeArm64 {
    arm64,
    arm64e,
}

impl SubtypeArm64 {
    pub fn new(cpu_subtype: CPUSubtype) -> Option<Self> {
        match cpu_subtype.masked() {
            CPU_SUBTYPE_ARM64_ALL => Some(Self::arm64),
            CPU_SUBTYPE_ARM64E => Some(Self::arm64e),
            _ => None,
        }
    }
}

impl Display for SubtypeArm64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}