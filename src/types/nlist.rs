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

    ///
    /// An integer specifying the number of the section that this symbol can be found in,
    /// or [`NO_SECT`] if the symbol is not to be found in any section of this image.
    /// The sections are contiguously numbered across segments, starting from 1,
    /// according to the order they appear in the [`LC_SEGMENT`] load commands.
    pub n_sect: u8,

    /// A 16-bit value providing additional information about the nature of this symbol for non-stab symbols.
    /// The reference flags can be accessed using the `REFERENCE_TYPE` mask (0xF) and are defined as follows:
    ///
    /// [`REFERENCE_FLAG_UNDEFINED_NON_LAZY`] (0x0) — This symbol is a reference to an external non-lazy (data) symbol.
    ///
    /// [`REFERENCE_FLAG_UNDEFINED_LAZY`] (0x1) — This symbol is a reference to an external lazy symbol—that is, to a function call.
    ///
    /// [`REFERENCE_FLAG_DEFINED`] (0x2) — This symbol is defined in this module.
    ///
    /// [`REFERENCE_FLAG_PRIVATE_DEFINED`] (0x3) — This symbol is defined in this module and is
    /// visible only to modules within this shared library.
    ///
    /// [`REFERENCE_FLAG_PRIVATE_UNDEFINED_NON_LAZY`] (0x4) — This symbol is defined in another module in this file,
    /// is a non-lazy (data) symbol, and is visible only to modules within this shared library.
    ///
    /// [`REFERENCE_FLAG_PRIVATE_UNDEFINED_LAZY`] (0x5) — This symbol is defined in another module in this file,
    /// is a lazy (function) symbol, and is visible only to modules within this shared library.
    ///
    /// Additionally, the following bits might also be set:
    ///
    /// [`REFERENCED_DYNAMICALLY`] (0x10) — Must be set for any defined symbol that is referenced by dynamic-loader
    /// APIs (such as dlsym and NSLookupSymbolInImage) and not ordinary undefined symbol references.
    /// The strip tool uses this bit to avoid removing symbols that must exist: If the symbol has this bit set,
    /// strip does not strip it.
    ///
    /// [`N_DESC_DISCARDED`] (0x20) — Sometimes used by the dynamic linker at runtime in a fully linked image.
    /// Do not set this bit in a fully linked image.
    ///
    /// [`N_NO_DEAD_STRIP`] (0x20) — When set in a relocatable object file (file type MH_OBJECT) on a defined symbol,
    /// indicates to the static linker to never dead-strip the symbol. (Note that the same bit (0x20) is used for two nonoverlapping purposes.)
    ///
    /// [`N_WEAK_REF`] (0x40) — Indicates that this undefined symbol is a weak reference.
    /// If the dynamic linker cannot find a definition for this symbol, it sets the address of
    /// this symbol to 0. The static linker sets this symbol given the appropriate weak-linking flags.
    ///
    /// [`N_WEAK_DEF`] (0x80) — Indicates that this symbol is a weak definition. If the static linker or the dynamic
    /// linker finds another (non-weak) definition for this symbol, the weak definition is ignored.
    /// Only symbols in a coalesced section can be marked as a weak definition.
    ///
    /// If this file is a two-level namespace image (that is, if the MH_TWOLEVEL flag of the mach_header
    /// structure is set), the high 8 bits of n_desc specify the number of the library in which
    /// this undefined symbol is defined. Use the macro `GET_LIBRARY_ORDINAL` to obtain this value and
    /// the macro `SET_LIBRARY_ORDINAL` to set it. Zero specifies the current image. 1 through 253 specify
    /// the library number according to the order of [`LC_LOAD_DYLIB`] commands in the file. The value 254 is used
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
    /// Common symbols must be of type [N_UNDF] and must have the [N_EXT] bit set.
    /// The n_value for a common symbol is the size (in bytes) of the data of the symbol.
    /// In C, a common symbol is a variable that is declared but not initialized in this file.
    /// Common symbols can appear only in MH_OBJECT Mach-O files.
    pub n_value: Nvalue,

    /// Depends on `n_strx`, `stroff` of [`LcSymtab`] (and image offset in file if that in fat file)
    pub name: Option<NlistStr>,
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

        // If `n_strx > 0`, name is not neccessarily have value. In case of stab it may be an empty string
        let name: Option<LcStr> = if n_strx > 0 {
            Some(NlistStr {
                reader: reader_clone,
                file_offset: stroff as u32 + n_strx,
            })
        } else {
            None
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

    /// Symbolic debugger symbols.  The comments give the conventional use for
    ///
    ///.stabs "n_name", n_type, n_sect, n_desc, n_value
    ///
    ///where n_type is the defined constant and not listed in the comment.  Other
    ///fields not listed are zero. n_sect is the section ordinal the entry is
    ///refering to.
    ///
    pub mod stab {
        /// global symbol: name,,NO_SECT,type,0
        pub const N_GSYM: u8 = 0x20;
        /// procedure name (f77 kludge): name,,NO_SECT,0,0
        pub const N_FNAME: u8 = 0x22;
        /// procedure: name,,n_sect,linenumber,address
        pub const N_FUN: u8 = 0x24;
        /// static symbol: name,,n_sect,type,address
        pub const N_STSYM: u8 = 0x26;
        /// .lcomm symbol: name,,n_sect,type,address
        pub const N_LCSYM: u8 = 0x28;
        /// begin nsect sym: 0,,n_sect,0,address
        pub const N_BNSYM: u8 = 0x2e;
        /// AST file path: name,,NO_SECT,0,0
        pub const N_AST: u8 = 0x32;
        /// emitted with gcc2_compiled and in gcc source
        pub const N_OPT: u8 = 0x3c;
        /// register sym: name,,NO_SECT,type,register
        pub const N_RSYM: u8 = 0x40;
        /// src line: 0,,n_sect,linenumber,address
        pub const N_SLINE: u8 = 0x44;
        /// end nsect sym: 0,,n_sect,0,address
        pub const N_ENSYM: u8 = 0x4e;
        /// structure elt: name,,NO_SECT,type,struct_offset
        pub const N_SSYM: u8 = 0x60;
        /// source file name: name,,n_sect,0,address
        pub const N_SO: u8 = 0x64;
        /// object file name: name,,0,0,st_mtime
        pub const N_OSO: u8 = 0x66;
        /// local sym: name,,NO_SECT,type,offset
        pub const N_LSYM: u8 = 0x80;
        /// include file beginning: name,,NO_SECT,0,sum
        pub const N_BINCL: u8 = 0x82;
        /// included file name: name,,n_sect,0,address
        pub const N_SOL: u8 = 0x84;
        /// compiler parameters: name,,NO_SECT,0,0
        pub const N_PARAMS: u8 = 0x86;
        /// compiler version: name,,NO_SECT,0,0
        pub const N_VERSION: u8 = 0x88;
        /// compiler -O level: name,,NO_SECT,0,0
        pub const N_OLEVEL: u8 = 0x8A;
        /// parameter: name,,NO_SECT,type,offset
        pub const N_PSYM: u8 = 0xa0;
        /// include file end: name,,NO_SECT,0,0
        pub const N_EINCL: u8 = 0xa2;
        /// alternate entry: name,,n_sect,linenumber,address
        pub const N_ENTRY: u8 = 0xa4;
        /// left bracket: 0,,NO_SECT,nesting level,address
        pub const N_LBRAC: u8 = 0xc0;
        /// deleted include file: name,,NO_SECT,0,sum
        pub const N_EXCL: u8 = 0xc2;
        /// right bracket: 0,,NO_SECT,nesting level,address
        pub const N_RBRAC: u8 = 0xe0;
        /// begin common: name,,NO_SECT,0,0
        pub const N_BCOMM: u8 = 0xe2;
        /// end common: name,,n_sect,0,0
        pub const N_ECOMM: u8 = 0xe4;
        /// end common (local name): 0,,n_sect,0,address
        pub const N_ECOML: u8 = 0xe8;
        /// second stab entry with length information
        pub const N_LENG: u8 = 0xfe;
    }
}

/// A byte value consisting of data accessed using four bit masks:
/// `N_STAB` (0xe0) — If any of these 3 bits are set, the symbol is a symbolic debugging table (stab) entry.
/// In that case, the entire n_type field is interpreted as a stabvalue.
/// See `/usr/include/mach-o/stab.h` for valid stab values.
///
/// [`N_PEXT`] (0x10) — If this bit is on, this symbol is marked as having limited global scope.
/// When the file is fed to the static linker, it clears the N_EXT bit for
/// each symbol with the `N_PEXT` bit set. (The ld option -keep_private_externs turns off this behavior.)
/// With OS X GCC, you can use the `__private_extern__` function attribute to set this bit.
///
/// [`N_TYPE`] (0x0e) — These bits define the type of the symbol.
///
/// [`N_EXT`] (0x01) — If this bit is on, this symbol is an external symbol, a symbol that is either
/// defined outside this file or that is defined in this file but can be referenced by other files.
///
/// Values for the N_TYPE field include:
///
/// [`N_UNDF`] (0x0) — The symbol is undefined. Undefined symbols are symbols referenced in this
/// module but defined in a different module. The n_sect field is set to NO_SECT.
///
/// [`N_ABS`] (0x2) — The symbol is absolute. The linker does not change the value of an absolute symbol.
/// The n_sect field is set to NO_SECT.
///
/// [`N_SECT`] (0xe) — The symbol is defined in the section number given in n_sect.
///
/// [`N_PBUD`] (0xc) — The symbol is undefined and the image is using a prebound value for the symbol.
/// The n_sect field is set to NO_SECT.
///
/// [`N_INDR`] (0xa) — The symbol is defined to be the same as another symbol.
/// The n_value field is an index into the string table specifying the name of the other symbol.
/// When that symbol is linked, both this and the other symbol have the same defined type and value.
#[derive(IOread, SizeWith)]
pub struct Ntype(pub u8);

use self::constants::*;
use constants::stab::*;

impl Ntype {
    pub fn stab_type(&self) -> Option<StabType> {
        StabType::from_raw(self.0)
    }

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

impl Ntype {
    pub fn options(&self) -> SymbolOptions {
        if let Some(stab) = self.stab_type() {
            return stab.options();
        }

        if self.is_undefined() {
            SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::Raw,
                n_value: NvalueOption::Raw,
            }
        } else if self.is_absolute() {
            SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::Raw,
                n_value: NvalueOption::Raw,
            }
        } else if self.is_defined_in_n_sect() {
            SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::Raw,
                n_value: NvalueOption::Address,
            }
        } else if self.is_prebound() {
            SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::Raw,
                n_value: NvalueOption::Raw,
            }
        } else {
            SymbolOptions {
                n_name: NnameOption::Raw,
                n_sect: NsectOption::Raw,
                n_desc: NdescOption::Raw,
                n_value: NvalueOption::Raw,
            }
        }
    }
}

#[derive(Debug)]
pub enum StabType {
    /// `N_GSYM`
    GlobalSymbol,
    /// `N_FNAME`
    ProcedureName,
    /// `N_FUN`
    Procedure,
    /// `N_STSYM`
    StaticSymbol,
    /// `N_LCSYM`
    LocalCommon,
    /// `N_BNSYM`
    BeginSection,
    /// `N_AST`
    AstFilePath,
    /// `N_OPT`
    Nopt,
    /// `N_RSYM`
    RegisterSymbol,
    /// `N_SLINE`
    SourceLine,
    /// `N_ENSYM`
    EndSection,
    /// `N_SSYM`
    Ssym,
    /// `N_SO`
    SourceFileName,
    /// `N_OSO`
    ObjectFileName,
    /// `N_LSYM`
    LocalSymbol,
    /// `N_BINCL`
    IncludeFileBeginning,
    /// `N_SOL`
    IncludedFileName,
    /// `N_PARAMS`
    CompilerParameters,
    /// `N_VERSION`
    CompilerVersion,
    /// `N_OLEVEL`
    CompilerOlevel,
    /// `N_PSYM`
    Parameter,
    /// `N_EINCL`
    IncludeFileEnd,
    /// `N_ENTRY`
    AlternateEntry,
    /// `N_LBRAC`
    LeftBracket,
    /// `N_EXCL`
    DeletedIncludeName,
    /// `N_RBRAC`
    RightBracket,
    /// `N_BCOMM`
    BeginCommon,
    /// `N_ECOMM`
    EndCommon,
    /// `N_ECOML`
    EndCommonLocalName,
    /// `N_LENG`
    LengthStabEntry,
}

impl StabType {
    pub(super) fn from_raw(raw: u8) -> Option<Self> {
        if raw & N_STAB == 0 {
            return None;
        }
        match raw {
            N_GSYM => Some(Self::GlobalSymbol),
            N_FNAME => Some(Self::ProcedureName),
            N_FUN => Some(Self::Procedure),
            N_STSYM => Some(Self::StaticSymbol),
            N_LCSYM => Some(Self::LocalCommon),
            N_BNSYM => Some(Self::BeginSection),
            N_AST => Some(Self::AstFilePath),
            N_OPT => Some(Self::Nopt),
            N_RSYM => Some(Self::RegisterSymbol),
            N_SLINE => Some(Self::SourceLine),
            N_ENSYM => Some(Self::EndSection),
            N_SSYM => Some(Self::Ssym),
            N_SO => Some(Self::SourceFileName),
            N_OSO => Some(Self::ObjectFileName),
            N_LSYM => Some(Self::LocalSymbol),
            N_BINCL => Some(Self::IncludeFileBeginning),
            N_SOL => Some(Self::IncludedFileName),
            N_PARAMS => Some(Self::CompilerParameters),
            N_VERSION => Some(Self::CompilerVersion),
            N_OLEVEL => Some(Self::CompilerOlevel),
            N_PSYM => Some(Self::Parameter),
            N_EINCL => Some(Self::IncludeFileEnd),
            N_ENTRY => Some(Self::AlternateEntry),
            N_LBRAC => Some(Self::LeftBracket),
            N_EXCL => Some(Self::DeletedIncludeName),
            N_RBRAC => Some(Self::RightBracket),
            N_BCOMM => Some(Self::BeginCommon),
            N_ECOMM => Some(Self::EndCommon),
            N_ECOML => Some(Self::EndCommonLocalName),
            N_LENG => Some(Self::LengthStabEntry),
            _ => None,
        }
    }
}

impl StabType {
    pub fn options(&self) -> SymbolOptions {
        match self {
            // global symbol: name,,NO_SECT,type,0
            StabType::GlobalSymbol => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::GlobalSymbolType,
                n_value: NvalueOption::None,
            },
            // procedure name (f77 kludge): name,,NO_SECT,0,0
            StabType::ProcedureName => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // procedure: name,,n_sect,linenumber,address
            StabType::Procedure => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::LineNumber,
                n_value: NvalueOption::Address,
            },
            // static symbol: name,,n_sect,type,address
            StabType::StaticSymbol => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::StaticSymbolType,
                n_value: NvalueOption::Address,
            },
            // .lcomm symbol: name,,n_sect,type,address
            StabType::LocalCommon => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::LocalCommonSymbolType,
                n_value: NvalueOption::Address,
            },
            // begin nsect sym: 0,,n_sect,0,address
            StabType::BeginSection => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Address,
            },
            // AST file path: name,,NO_SECT,0,0
            StabType::AstFilePath => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // emitted with gcc2_compiled and in gcc source
            StabType::Nopt => SymbolOptions {
                n_name: NnameOption::Unknown,
                n_sect: NsectOption::Unknown,
                n_desc: NdescOption::Unknown,
                n_value: NvalueOption::Unknown,
            },
            // register sym: name,,NO_SECT,type,register
            StabType::RegisterSymbol => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::RegisterType,
                n_value: NvalueOption::Register,
            },
            // src line: 0,,n_sect,linenumber,address
            StabType::SourceLine => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::LineNumber,
                n_value: NvalueOption::Address,
            },
            // end nsect sym: 0,,n_sect,0,address
            StabType::EndSection => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::None,
                n_value:NvalueOption::Address,
            },
            // structure elt: name,,NO_SECT,type,struct_offset
            StabType::Ssym => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::StructureEltType,
                n_value: NvalueOption::StructOffset,
            },
            // source file name: name,,n_sect,0,address
            StabType::SourceFileName => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Address,
            },
            // object file name: name,,0,0,st_mtime
            StabType::ObjectFileName => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Zero,
                n_desc: NdescOption::None,
                n_value: NvalueOption::LastModTime,
            },
            // local sym: name,,NO_SECT,type,offset
            StabType::LocalSymbol => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::SymbolType,
                n_value: NvalueOption::Offset,
            },
            // include file beginning: name,,NO_SECT,0,sum
            StabType::IncludeFileBeginning => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Sum,
            },
            // #included file name: name,,n_sect,0,address
            StabType::IncludedFileName => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Address,
            },
            // compiler parameters: name,,NO_SECT,0,0
            StabType::CompilerParameters => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // compiler version: name,,NO_SECT,0,0
            StabType::CompilerVersion => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // compiler -O level: name,,NO_SECT,0,0 */
            StabType::CompilerOlevel => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // parameter: name,,NO_SECT,type,offset
            StabType::Parameter => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::ParameterType,
                n_value: NvalueOption::Offset,
            },
            // include file end: name,,NO_SECT,0,0
            StabType::IncludeFileEnd => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // alternate entry: name,,n_sect,linenumber,address
            StabType::AlternateEntry => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::LineNumber,
                n_value: NvalueOption::Address,
            },
            // left bracket: 0,,NO_SECT,nesting level,address
            StabType::LeftBracket => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::None,
                n_desc: NdescOption::NestingLevel,
                n_value: NvalueOption::Address,
            },
            // deleted include file: name,,NO_SECT,0,sum
            StabType::DeletedIncludeName => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Sum,
            },
            // right bracket: 0,,NO_SECT,nesting level,address
            StabType::RightBracket => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::None,
                n_desc: NdescOption::NestingLevel,
                n_value: NvalueOption::Address,
            },
            // begin common: name,,NO_SECT,0,0
            StabType::BeginCommon => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // end common: name,,n_sect,0,0
            StabType::EndCommon => SymbolOptions {
                n_name: NnameOption::Some,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::None,
                n_value: NvalueOption::None,
            },
            // end common (local name): 0,,n_sect,0,address
            StabType::EndCommonLocalName => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::Some,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Address,
            },
            // second stab entry with length information
            StabType::LengthStabEntry => SymbolOptions {
                n_name: NnameOption::None,
                n_sect: NsectOption::None,
                n_desc: NdescOption::None,
                n_value: NvalueOption::Length,
            },
        }
    }
}

/// .stabs "n_name", n_type (always constant), n_sect, n_desc, n_value
pub struct SymbolOptions {
    pub n_name: NnameOption,
    pub n_sect: NsectOption,
    pub n_desc: NdescOption,
    pub n_value: NvalueOption,
}

pub enum NnameOption {
    None,
    Unknown,
    Some,
    Raw,
}
pub enum NsectOption {
    None,
    Unknown,
    Some,
    Zero,
    Raw,
}

pub enum NdescOption {
    None,
    Unknown,
    GlobalSymbolType,
    StaticSymbolType,
    LocalCommonSymbolType,
    LineNumber,
    RegisterType,
    StructureEltType,
    SymbolType,
    ParameterType,
    NestingLevel,
    Raw,
}

pub enum NvalueOption {
    None,
    Unknown,
    Address,
    Register,
    StructOffset,
    LastModTime,
    Offset,
    Sum,
    Length,
    Raw,
}
