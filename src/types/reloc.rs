//! References:
//! <https://llvm.org/doxygen/namespacellvm_1_1MachO.html#ad23021ed657c8b0f302154585b0fd3bfa75544865ca16caaf6ebb768f93879546>
//! <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/reloc.h.auto.html>
//! <https://opensource.apple.com/source/clang/clang-703.0.31/src/lib/Target/ARM/MCTargetDesc/ARMMachObjectWriter.cpp.auto.html>

use scroll::*;

use self::constants::R_SCATTERED;

pub mod constants {
    /// Absolute relocation type for Mach-O files
    pub const R_ABS: u8 = 0;

    /// mask to be applied to the r_address field
    /// of a relocation_info structure to tell that
    /// is is really a scattered_relocation_info
    /// stucture
    pub const R_SCATTERED: u32 = 0x80000000;

    /// Absolute address
    pub const X86_64_RELOC_UNSIGNED: u8 = 0;
    /// Signed 32-bit displacement
    pub const X86_64_RELOC_SIGNED: u8 = 1;
    /// A CALL/JMP instruction with 32-bit displacement
    pub const X86_64_RELOC_BRANCH: u8 = 2;
    /// A MOVQ load of a GOT entry
    pub const X86_64_RELOC_GOT_LOAD: u8 = 3;
    /// Other GOT references
    pub const X86_64_RELOC_GOT: u8 = 4;
    /// Must be followed by a X86_64_RELOC_UNSIGNED relocation
    pub const X86_64_RELOC_SUBTRACTOR: u8 = 5;
    /// for signed 32-bit displacement with a -1 addend
    pub const X86_64_RELOC_SIGNED_1: u8 = 6;
    /// for signed 32-bit displacement with a -2 addend
    pub const X86_64_RELOC_SIGNED_2: u8 = 7;
    /// for signed 32-bit displacement with a -4 addend
    pub const X86_64_RELOC_SIGNED_4: u8 = 8;
    /// https://llvm.org/doxygen/namespacellvm_1_1MachO.html#ad23021ed657c8b0f302154585b0fd3bfa75544865ca16caaf6ebb768f93879546
    pub const X86_64_RELOC_TLV: u8 = 9;

    // x86
    pub const GENERIC_RELOC_VANILLA: u8 = 0;
    pub const GENERIC_RELOC_PAIR: u8 = 1;
    pub const GENERIC_RELOC_SECTDIFF: u8 = 2;
    pub const GENERIC_RELOC_PB_LA_PTR: u8 = 3;
    pub const GENERIC_RELOC_LOCAL_SECTDIFF: u8 = 4;
    pub const GENERIC_RELOC_TLV: u8 = 5;

    // arm
    pub const ARM_RELOC_VANILLA: u8 = GENERIC_RELOC_VANILLA;
    pub const ARM_RELOC_PAIR: u8 = GENERIC_RELOC_PAIR;
    pub const ARM_RELOC_SECTDIFF: u8 = GENERIC_RELOC_SECTDIFF;
    pub const ARM_RELOC_LOCAL_SECTDIFF: u8 = 3;
    pub const ARM_RELOC_PB_LA_PTR: u8 = 4;
    pub const ARM_RELOC_BR24: u8 = 5;
    pub const ARM_THUMB_RELOC_BR22: u8 = 6;

    /// Obsolete
    pub const ARM_THUMB_32BIT_BRANCH: u8 = 7;
    pub const ARM_RELOC_HALF: u8 = 8;
    pub const ARM_RELOC_HALF_SECTDIFF: u8 = 9;

    /// For pointers.
    pub const ARM64_RELOC_UNSIGNED: u8 = 0;
    /// Must be followed by an ARM64_RELOC_UNSIGNED
    pub const ARM64_RELOC_SUBTRACTOR: u8 = 1;
    /// A B/BL instruction with 26-bit displacement.
    pub const ARM64_RELOC_BRANCH26: u8 = 2;
    /// PC-rel distance to page of target.
    pub const ARM64_RELOC_PAGE21: u8 = 3;
    /// Offset within page, scaled by r_length.
    pub const ARM64_RELOC_PAGEOFF12: u8 = 4;
    /// PC-rel distance to page of GOT slot.
    pub const ARM64_RELOC_GOT_LOAD_PAGE21: u8 = 5;
    /// Offset within page of GOT slot, scaled by r_length.
    pub const ARM64_RELOC_GOT_LOAD_PAGEOFF12: u8 = 6;
    /// For pointers to GOT slots.
    pub const ARM64_RELOC_POINTER_TO_GOT: u8 = 7;
    /// PC-rel distance to page of TLVP slot.
    pub const ARM64_RELOC_TLVP_LOAD_PAGE21: u8 = 8;
    /// Offset within page of TLVP slot, scaled by r_length.
    pub const ARM64_RELOC_TLVP_LOAD_PAGEOFF12: u8 = 9;
    /// Must be followed by ARM64_RELOC_PAGE21 or ARM64_RELOC_PAGEOFF12.
    pub const ARM64_RELOC_ADDEND: u8 = 10;
}

/// Represents `relocation_info`
/// Format of a relocation entry of a Mach-O file.  Modified from the 4.3BSD
/// format.  The modifications from the original format were changing the value
/// of the r_symbolnum field for "local" (r_extern == 0) relocation entries.
/// This modification is required to support symbols in an arbitrary number of
/// sections not just the three sections (text, data and bss) in a 4.3BSD file.
/// Also the last 4 bits have had the r_type tag added to them.
#[derive(IOread, SizeWith)]
pub struct RelocationInfo {
    /// offset in the section to what is being relocated
    pub r_address: i32,
    /// See functions
    pub r_bitfield: u32,
}

impl RelocationInfo {
    /// Symbol index if r_extern == 1 or section ordinal if r_extern == 0
    /// r_symbolnum:24
    pub fn r_symbolnum(&self) -> usize {
        (self.r_bitfield & 0x00ff_ffffu32) as usize
    }

    /// Was relocated pc relative already
    // r_pcrel:1
    pub fn r_pcrel(&self) -> u8 {
        ((self.r_bitfield & 0x0100_0000u32) >> 24) as u8
    }

    /// 0=byte, 1=word, 2=long, 3=quad
    /// r_length:2
    pub fn r_length(&self) -> u8 {
        ((self.r_bitfield & 0x0600_0000u32) >> 25) as u8
    }

    /// Does not include value of sym referenced
    /// r_extern:1
    pub fn r_extern(&self) -> u8 {
        ((self.r_bitfield & 0x0800_0000) >> 27) as u8
    }

    /// If not 0, machine specific relocation type
    /// r_type:4
    pub fn r_type(&self) -> u8 {
        ((self.r_bitfield & 0xf000_0000) >> 28) as u8
    }

    pub fn is_scattered(&self) -> bool {
        self.r_address as u32 & R_SCATTERED == R_SCATTERED
    }
}
