use super::constants::*;

pub fn masked_cpu_subtype(cpusubtype: CPUSubtype) -> CPUSubtype {
    cpusubtype & !CPU_SUBTYPE_MASK
}

pub fn feature_flags(cpusubtype: CPUSubtype) -> u32 {
    (cpusubtype & CPU_SUBTYPE_MASK) >> 24
}

pub fn is_64(cputype: CPUType) -> bool {
    (cputype & CPU_ARCH_ABI64) == CPU_ARCH_ABI64
}