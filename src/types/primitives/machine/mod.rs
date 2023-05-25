//! <https://opensource.apple.com/source/xnu/xnu-6153.11.26/osfmk/mach/machine.h.auto.html>

use std::fmt::{Debug, Display};

pub mod cpu;
pub use cpu::*;

pub struct Machine {
    cpu_type: CPUType,
    cpu_subtype: CPUSubtype,
}

impl Machine {
    pub fn new(cpu_type: CPUType, cpu_subtype: CPUSubtype) -> Self {
        Machine {
            cpu_type,
            cpu_subtype,
        }
    }
}

impl Machine {
    /// If returned None, use the raw values - [Machine]'s `cpu_type` and `cpu_subtype`
    pub fn cpu(&self) -> Option<CPU> {
        CPU::new(self.cpu_type, self.cpu_subtype)
    }
}

use self::cpu_constants::*;

#[allow(non_camel_case_types)]
pub enum CPU {
    x86_64(SubtypeX86_64),
    arm64(SubtypeArm64),
}

impl CPU {
    pub fn new(cpu_type: CPUType, cpu_subtype: CPUSubtype) -> Option<Self> {
        match cpu_type {
            CPU_TYPE_X86_64 => match SubtypeX86_64::new(cpu_subtype) {
                Some(s) => Some(Self::x86_64(s)),
                None => None,
            },
            CPU_TYPE_ARM64 => match SubtypeArm64::new(cpu_subtype) {
                Some(s) => Some(Self::arm64(s)),
                None => None,
            },
            _ => None,
        }
    }
}

impl Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::x86_64(arg0) => write!(f, "{}", arg0),
            Self::arm64(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl Display for CPU {
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