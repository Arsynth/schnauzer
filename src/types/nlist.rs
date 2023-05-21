//! <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/nlist.h.auto.html>

use super::U64U32;
use crate::LcStr;
use crate::RcReader;
use crate::Result;

use scroll::IOread;
use scroll::SizeWith;

type NlistStr = LcStr;
type Nvalue = U64U32;

/// `nlist`
/// Describes an entry in the symbol table. Declared in `/usr/include/mach-o/nlist.h`.
#[repr(C)]
pub struct Nlist {
    /// In the original `nlist` struct this field is uniun - `n_un`
    /// A union that holds an index into the string table, n_strx. To specify an empty string (""),
    /// set this value to 0. The n_name field is not used in Mach-O files.
    pub n_strx: u32,

    /// See `Ntype`
    pub n_type: Ntype,

    /// An integer specifying the number of the section that this symbol can be found in,
    /// or `NO_SECT` if the symbol is not to be found in any section of this image.
    /// The sections are contiguously numbered across segments, starting from 1,
    /// according to the order they appear in the `LC_SEGMENT` load commands.
    pub n_sect: u8,

    /// A 16-bit value providing additional information about the nature of this symbol for non-stab symbols.
    /// The reference flags can be accessed using the `REFERENCE_TYPE` mask (0xF) and are defined as follows:
    ///
    /// `REFERENCE_FLAG_UNDEFINED_NON_LAZY` (0x0) — This symbol is a reference to an external non-lazy (data) symbol.
    ///
    /// `REFERENCE_FLAG_UNDEFINED_LAZY` (0x1) — This symbol is a reference to an external lazy symbol—that is, to a function call.
    ///
    /// `REFERENCE_FLAG_DEFINED` (0x2) — This symbol is defined in this module.
    ///
    /// `REFERENCE_FLAG_PRIVATE_DEFINED` (0x3) — This symbol is defined in this module and is
    /// visible only to modules within this shared library.
    ///
    /// `REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY` (0x4) — This symbol is defined in another module in this file,
    /// is a non-lazy (data) symbol, and is visible only to modules within this shared library.
    ///
    /// `REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY` (0x5) — This symbol is defined in another module in this file,
    /// is a lazy (function) symbol, and is visible only to modules within this shared library.
    ///
    /// Additionally, the following bits might also be set:
    ///
    /// `REFERENCED_DYNAMICALLY` (0x10) — Must be set for any defined symbol that is referenced by dynamic-loader
    /// APIs (such as dlsym and NSLookupSymbolInImage) and not ordinary undefined symbol references.
    /// The strip tool uses this bit to avoid removing symbols that must exist: If the symbol has this bit set,
    /// strip does not strip it.
    ///
    /// `N_DESC_DISCARDED` (0x20) — Sometimes used by the dynamic linker at runtime in a fully linked image.
    /// Do not set this bit in a fully linked image.
    ///
    /// `N_NO_DEAD_STRIP` (0x20) — When set in a relocatable object file (file type MH_OBJECT) on a defined symbol,
    /// indicates to the static linker to never dead-strip the symbol. (Note that the same bit (0x20) is used for two nonoverlapping purposes.)
    ///
    /// `N_WEAK_REF` (0x40) — Indicates that this undefined symbol is a weak reference.
    /// If the dynamic linker cannot find a definition for this symbol, it sets the address of
    /// this symbol to 0. The static linker sets this symbol given the appropriate weak-linking flags.
    ///
    /// `N_WEAK_DEF` (0x80) — Indicates that this symbol is a weak definition. If the static linker or the dynamic
    /// linker finds another (non-weak) definition for this symbol, the weak definition is ignored.
    /// Only symbols in a coalesced section can be marked as a weak definition.
    ///
    /// If this file is a two-level namespace image (that is, if the MH_TWOLEVEL flag of the mach_header
    /// structure is set), the high 8 bits of n_desc specify the number of the library in which
    /// this undefined symbol is defined. Use the macro `GET_LIBRARY_ORDINAL` to obtain this value and
    /// the macro `SET_LIBRARY_ORDINAL` to set it. Zero specifies the current image. 1 through 253 specify
    /// the library number according to the order of `LC_LOAD_DYLIB` commands in the file. The value 254 is used
    /// for undefined symbols that are to be dynamically looked up (supported only in OS X v10.3 and later).
    /// For plug–ins that load symbols from the executable program they are linked against, 255 specifies
    ///  the executable image. For flat namespace images, the high 8 bits must be 0.
    pub n_desc: u16,

    /// An integer that contains the value of the symbol. The format of this value is different for
    ///  each type of symbol table entry (as specified by the n_type field).
    /// For the `N_SECT` symbol type, `n_value` is the address of the symbol. See the description of the n_type
    /// field for information on other possible values.
    ///
    /// ### Discussion
    ///
    /// Common symbols must be of type N_UNDF and must have the N_EXT bit set.
    /// The n_value for a common symbol is the size (in bytes) of the data of the symbol.
    /// In C, a common symbol is a variable that is declared but not initialized in this file.
    /// Common symbols can appear only in MH_OBJECT Mach-O files.
    pub n_value: Nvalue,

    /// Depends on `n_strx`, `stroff` of `LcSymtab` [and image offset in file if that in fat file]
    pub name: NlistStr,
}

impl Nlist {
    pub(super) fn parse(
        reader: RcReader,
        stroff: u64,
        is_64: bool,
        endian: scroll::Endian,
    ) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();

        let n_strx: u32 = reader_mut.ioread_with(endian)?;
        let n_type: Ntype = reader_mut.ioread_with(endian)?;
        let n_sect: u8 = reader_mut.ioread_with(endian)?;
        let n_desc: u16 = reader_mut.ioread_with(endian)?;
        let n_value: Nvalue = if is_64 {
            let val: u64 = reader_mut.ioread_with(endian)?;
            Nvalue::U64(val)
        } else {
            let val: u32 = reader_mut.ioread_with(endian)?;
            Nvalue::U32(val)
        };
        let name = NlistStr {
            reader: reader_clone,
            file_offset: stroff as u32 + n_strx,
        };

        Ok(Self {
            n_strx,
            n_type,
            n_sect,
            n_desc,
            n_value,
            name,
        })
    }
}

pub mod constants {
    pub const N_STAB: u8 = 0xe0;
    pub const N_PEXT: u8 = 0x10;
    pub const N_TYPE: u8 = 0x0e;
    pub const N_EXT: u8 = 0x01;

    pub const N_UNDF: u8 = 0x0;
    pub const N_ABS: u8 = 0x2;
    pub const N_SECT: u8 = 0xe;
    pub const N_PBUD: u8 = 0xc;
    pub const N_INDR: u8 = 0xa;
}

/// A byte value consisting of data accessed using four bit masks:
/// `N_STAB` (0xe0) — If any of these 3 bits are set, the symbol is a symbolic debugging table (stab) entry.
/// In that case, the entire n_type field is interpreted as a stabvalue.
/// See `/usr/include/mach-o/stab.h` for valid stab values.
///
/// `N_PEXT` (0x10) — If this bit is on, this symbol is marked as having limited global scope.
/// When the file is fed to the static linker, it clears the N_EXT bit for
/// each symbol with the `N_PEXT` bit set. (The ld option -keep_private_externs turns off this behavior.)
/// With OS X GCC, you can use the `__private_extern__` function attribute to set this bit.
///
/// `N_TYPE` (0x0e) — These bits define the type of the symbol.
///
/// `N_EXT` (0x01) — If this bit is on, this symbol is an external symbol, a symbol that is either
/// defined outside this file or that is defined in this file but can be referenced by other files.
///
/// Values for the N_TYPE field include:
///
/// `N_UNDF` (0x0) — The symbol is undefined. Undefined symbols are symbols referenced in this
/// module but defined in a different module. The n_sect field is set to NO_SECT.
///
/// `N_ABS` (0x2) — The symbol is absolute. The linker does not change the value of an absolute symbol.
/// The n_sect field is set to NO_SECT.
///
/// `N_SECT` (0xe) — The symbol is defined in the section number given in n_sect.
///
/// `N_PBUD` (0xc) — The symbol is undefined and the image is using a prebound value for the symbol.
/// The n_sect field is set to NO_SECT.
///
/// `N_INDR` (0xa) — The symbol is defined to be the same as another symbol.
/// The n_value field is an index into the string table specifying the name of the other symbol.
/// When that symbol is linked, both this and the other symbol have the same defined type and value.
#[derive(IOread, SizeWith)]
pub struct Ntype(pub u8);

use self::constants::*;

impl Ntype {
    pub fn is_stab(&self) -> bool {
        self.0 & N_STAB > 0
    }

    pub fn is_private_external(&self) -> bool {
        self.0 & N_PEXT > 0
    }

    pub fn is_external(&self) -> bool {
        self.0 & N_EXT > 0
    }

    pub fn is_undefined(&self) -> bool {
        self.0 & N_TYPE == N_UNDF 
    }

    pub fn is_absolute(&self) -> bool {
        self.0 & N_TYPE == N_ABS
    }

    pub fn is_defined_in_n_sect(&self) -> bool {
        self.0 & N_TYPE == N_SECT
    }

    pub fn is_prebound(&self) -> bool {
        self.0 & N_TYPE == N_PBUD
    }

    /// If `true`, the n_value field is an index into the string table specifying the name of the other symbol.
    pub fn is_indirect(&self) -> bool {
        self.0 & N_TYPE == N_INDR
    }
}
