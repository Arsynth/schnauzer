pub const BYTES_PER_MAGIC: usize = 4;
pub const BYTES_PER_FAT_HEADER: usize = 8;
pub const BYTES_PER_FAT_ARCH: usize = 20;
pub const BYTES_PER_LOAD_COMMAND: usize = 8;
pub const BYTES_PER_SECTION32: usize = 68;
pub const BYTES_PER_SECTION64: usize = 80;
pub const BYTES_PER_NLIST32: usize = 12;
pub const BYTES_PER_NLIST64: usize = 16;

/// <https://opensource.apple.com/source/xnu/xnu-4570.41.2/osfmk/mach/machine.h.auto.html>

pub const CPU_SUBTYPE_MASK: u32 = 0xff000000;
pub const CPU_SUBTYPE_LIB64: u32 = 0x80000000;

pub const CPU_ARCH_ABI64: i32 = 0x01000000;