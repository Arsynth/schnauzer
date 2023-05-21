//! <https://github.com/Arsynth/osx-abi-macho-file-format-reference>

use super::primitives::*;

/// `dylib_table_of_contents`
/// Describes an entry in the table of contents of a dynamic shared library. Declared in /usr/include/mach-o/loader.h.
pub struct TableOfContents {
    /// An index into the symbol table indicating the defined external symbol to which this entry refers
    pub symbol_index: u32,
    /// An index into the module table indicating the module in which this defined external symbol is defined
    pub module_index: u32,
}

/// Both `dylib_module` and `dylib_module_64`
/// Describes a module table entry for a dynamic shared library for 32-bit architectures. 
/// Declared in /usr/include/mach-o/loader.h. See also `dylib_module_64`.
pub struct Module {
    /// An index to an entry in the string table indicating the name of the module
    pub module_name: u32,
    /// The index into the symbol table of the first defined external symbol provided by this module
    pub iextdefsym: u32,
    /// The number of defined external symbols provided by this module
    pub nextdefsym: u32,
    /// The index into the external reference table of the first entry provided by this module
    pub irefsym: u32,
    /// The number of external reference entries provided by this module
    pub nrefsym: u32,
    /// The index into the symbol table of the first local symbol provided by this module
    pub ilocalsym: u32,
    /// The number of local symbols provided by this module
    pub nlocalsym: u32,
    /// The index into the external relocation table of the first entry provided by this module
    pub iextrel: u32,
    /// The number of entries in the external relocation table that are provided by this module
    pub nextrel: u32,
    /// Contains both the index into the module initialization section (the low 16 bits)
    /// and the index into the module termination section (the high 16 bits) to the pointers for this module
    pub iinit_iterm: u32,
    /// Contains both the number of pointers in the module initialization (the low 16 bits)
    /// and the number of pointers in the module termination section (the high 16 bits) for this module
    pub ninit_nterm: u32,
    /// The statically linked address of the start of the data for this module
    /// in the __module_info section in the `__OBJC` segment
    pub objc_module_info_addr: u32,
    /// The number of bytes of data for this module that are used in the __module_info section in the __OBJC segment.
    pub objc_module_info_size: U64U32,
}

/// `dylib_reference`
/// Defines the attributes of an external reference table entry for the external reference entries 
/// provided by a module in a shared library. Declared in /usr/include/mach-o/loader.h.
pub struct Reference {
    pub value: ReferenceBitField,
    
}

pub struct ReferenceBitField(pub u32);

/// Represents a bitfield - `uint32_t isym : 24, flags : 8;`
impl ReferenceBitField {
    /// An index into the symbol table for the symbol being referenced
    pub fn isym() -> u32 {
        todo!()
    }
    /// A constant for the type of reference being made.
    /// Use the same `REFERENCE_FLAG` constants as described in the `nlist` structure description
    pub fn flags() -> u32 {
        todo!()
    }
}


/// `relocation_info`
/// Describes an item in the file that uses an address that needs to be updated
/// when the address is changed. Declared in /usr/include/mach-o/reloc.h
pub struct RelocationInfo {
    /// In MH_OBJECT files, this is an offset from the start of the section to the item
    /// containing the address requiring relocation.
    /// If the high bit of this field is set (which you can check using the R_SCATTERED bit mask),
    /// the relocation_info structure is actually a scattered_relocation_info structure.
    /// In images used by the dynamic linker, this is an offset from the virtual memory address of the data
    /// of the first segment_command that appears in the file (not necessarily the one with the lowest address).
    /// For images with the MH_SPLIT_SEGS flag set, this is an offset from the virtual memory address of data of the first read/write segment_command.
    pub r_address: i32,
    /// Indicates either an index into the symbol table (when the r_extern field is set to 1)
    /// or a section number (when the r_extern field is set to 0).
    /// As previously mentioned, sections are ordered from 1 to 255 in the order in which they appear
    /// in the LC_SEGMENT load commands. This field is set to R_ABS for relocation entries for absolute symbols, which need no relocation.
    pub r_symbolnum: u32,
    /// Indicates whether the item containing the address to be relocated is part of a CPU instruction
    /// that uses PC-relative addressing
    /// For addresses contained in PC-relative instructions, the CPU adds the address of the instruction
    /// to the address contained in the instruction.
    pub r_pcrel: u32,
    /// ndicates the length of the item containing the address to be relocated.
    /// The following table lists r_length values and the corresponding address length.
    ///
    /// 0 - 1 byte
    /// 1 - 2 bytes
    /// 2 - 4 bytes
    /// 3 - 8 bytes. See description for the `PPC_RELOC_BR14` `r_type` in `scattered_relocation_info`
    pub r_length: u32,
    /// Indicates whether the `r_symbolnum` field is an index into the symbol table (1) or a section number (0).
    pub r_extern: u32,
    /// For the x86 environment, the r_type field may contain any of these values:
    /// `GENERIC_RELOC_VANILLA` — A generic relocation entry for both addresses contained in data and addresses
    /// contained in CPU instructions.
    /// `GENERIC_RELOC_PAIR` — The second relocation entry of a pair.
    /// `GENERIC_RELOC_SECTDIFF` — A relocation entry for an item that contains the difference of two section addresses.
    /// This is generally used for position-independent code generation. GENERIC_RELOC_SECTDIFF contains the address
    /// from which to subtract; it must be followed by a GENERIC_RELOC_PAIR containing the address to subtract.
    /// `GENERIC_RELOC_LOCAL_SECTDIFF` — Similar to GENERIC_RELOC_SECTDIFF except that this entry refers
    /// specifically to the address in this item. If the address is that of a globally visible coalesced symbol, this relocation entry does not change if the symbol is overridden. This is used to associate stack unwinding information with the object code this relocation entry describes.
    /// `GENERIC_RELOC_PB_LA_PTR` — A relocation entry for a prebound lazy pointer. This is always a scattered relocation entry.
    /// The r_value field contains the non-prebound value of the lazy pointer.
    ///
    /// For the x86-64 environment, the r_type field may contain any of these values:
    /// `X86_64_RELOC_BRANCH` — A CALL/JMP instruction with 32-bit displacement.
    /// `X86_64_RELOC_GOT_LOAD` — A MOVQ load of a GOT entry.
    /// `X86_64_RELOC_GOT` — Other GOT references.
    /// `X86_64_RELOC_SIGNED` — Signed 32-bit displacement.
    /// `X86_64_RELOC_UNSIGNED` — Absolute address.
    /// `X86_64_RELOC_SUBTRACTOR` — Must be followed by a X86_64_RELOC_UNSIGNED relocation.
    ///
    /// For PowerPC environments, the r_type field is usually `PPC_RELOC_VANILLA` for addresses contained in data.
    /// Relocation entries for addresses contained in CPU instructions are described by other `r_type` values:
    /// PPC_RELOC_PAIR—The second relocation entry of a pair. A PPC_RELOC_PAIR entry must follow each of the other relocation entry types, except for PPC_RELOC_VANILLA, PPC_RELOC_BR14, PPC_RELOC_BR24, and PPC_RELOC_PB_LA_PTR.
    /// `PPC_RELOC_BR14` — The instruction contains a 14-bit branch displacement. If the r_length is 3, the branch was statically predicted by setting or clearing the Y bit depending on the sign of the displacement or the opcode.
    /// `PPC_RELOC_BR24` — The instruction contains a 24-bit branch displacement.
    /// `PPC_RELOC_HI16` — The instruction contains the high 16 bits of a relocatable expression. 
    /// The next relocation entry must be a PPC_RELOC_PAIR specifying the low 16 bits of the expression 
    /// in the low 16 bits of the r_value field.
    /// `PPC_RELOC_LO16` — The instruction contains the low 16 bits of an address. 
    /// The next relocation entry must be a PPC_RELOC_PAIR specifying the high 16 bits of the expression 
    /// in the low (not the high) 16 bits of the r_value field.
    /// `PPC_RELOC_HA16` — Same as the PPC_RELOC_HI16 except the low 16 bits and the high 16 bits 
    /// are added together with the low 16 bits sign-extended first. This means if bit 15 of the low 16 bits is set, 
    /// the high 16 bits stored in the instruction are adjusted.
    /// `PPC_RELOC_LO14` — Same as PPC_RELOC_LO16 except that the low 2 bits are not stored 
    /// in the CPU instruction and are always 0. PPC_RELOC_LO14 is used in 64-bit load/store instructions.
    /// `PPC_RELOC_SECTDIFF` — A relocation entry for an item that contains the difference of two section addresses. 
    /// This is generally used for position-independent code generation. `PPC_RELOC_SECTDIFF` contains the address 
    /// from which to subtract; it must be followed by a PPC_RELOC_PAIR containing the section address to subtract.
    /// `PPC_RELOC_LOCAL_SECTDIFF` — Similar to `PPC_RELOC_SECTDIFF` except that this entry refers specifically 
    /// to the address in this item. If the address is that of a globally visible coalesced symbol, 
    /// this relocation entry does not change if the symbol is overridden. 
    /// This is used to associate stack unwinding information with the object code this relocation entry describes
    /// `PPC_RELOC_PB_LA_PTR` — A relocation entry for a prebound lazy pointer. This is always a scattered relocation entry. 
    /// The r_value field contains the non-prebound value of the lazy pointer.
    /// `PPC_RELOC_HI16_SECTDIFF` — Section difference form of `PPC_RELOC_HI16`.
    /// `PPC_RELOC_LO16_SECTDIFF` — Section difference form of `PPC_RELOC_LO16`.
    /// `PPC_RELOC_HA16_SECTDIFF` — Section difference form of `PPC_RELOC_HA16`.
    /// `PPC_RELOC_JBSR` — A relocation entry for the assembler synthetic opcode jbsr, 
    /// which is a 24-bit branch-and-link instruction using a branch island. 
    /// The branch displacement is assembled to the branch island address and the relocation entry indicates 
    /// the actual target symbol. If the linker is able to make the branch reach the actual target symbol, it does. 
    /// Otherwise, the branch is relocated to the branch island.
    /// `PPC_RELOC_LO14_SECTDIFF` — Section difference form of `PPC_RELOC_LO14`.
    pub r_type: u32,
}

/// `scattered_relocation_info`
/// Describes an item in the file—using a nonzero constant in its relocatable expression or 
/// two addresses in its relocatable expression—that needs to be updated if the addresses that it uses are changed. 
/// This information is needed to reconstruct the addresses that make up the relocatable expression’s value in order
/// to change the addresses independently of each other. Declared in /usr/include/mach-o/reloc.h.
pub struct ScatteredRelocationInfo {
    pub bit_field: ScatteredRelocationInfoBitField,
    pub r_value: i32,
}

pub struct ScatteredRelocationInfoBitField(pub u32);

/* 
struct scattered_relocation_info {
#ifdef __BIG_ENDIAN__
   uint32_t r_scattered : 1, r_pcrel : 1, r_length : 2, r_type : 4, r_address : 24;
   int32_t r_value;
#endif /* __BIG_ENDIAN__ */
#ifdef __LITTLE_ENDIAN__
   uint32_t r_address : 24, r_type : 4, r_length : 2, r_pcrel : 1, r_scattered : 1;
   int32_t r_value;
#endif  /* __LITTLE_ENDIAN__ */
};
*/
impl ScatteredRelocationInfoBitField {
    pub fn r_scattered() -> u32 {
        todo!()
    }
    pub fn r_pcrel() -> u32 {
        todo!()
    }
    pub fn r_length() -> u32 {
        todo!()
    }
    pub fn r_type() -> u32 {
        todo!()
    }
    pub fn r_address() -> u32 {
        todo!()
    }
}