pub const BYTES_PER_MAGIC: usize = 4;
pub const BYTES_PER_FAT_HEADER: usize = 8;
pub const BYTES_PER_FAT_ARCH: usize = 20;
pub const BYTES_PER_LOAD_COMMAND: usize = 8;

pub type CPUType = u32;
pub type CPUSubtype = u32;
pub type VmProt = i32;

pub type LoadCommandType = u32;

/// <https://opensource.apple.com/source/xnu/xnu-4570.41.2/osfmk/mach/machine.h.auto.html>

pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;
pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000;

pub const CPU_ARCH_ABI64: CPUType = 0x01000000;
