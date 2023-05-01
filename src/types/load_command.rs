use super::constants::*;
use super::RcReader;
use super::Result;
use scroll::SizeWith;
use scroll::{IOread, Endian};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

pub const LC_REQ_DYLD: u32 = 0x80000000;

pub const LC_SEGMENT: u32 = 0x1;
pub const LC_SYMTAB: u32 = 0x2;
pub const LC_SYMSEG: u32 = 0x3;
pub const LC_THREAD: u32 = 0x4;
pub const LC_UNIXTHREAD: u32 = 0x5;

/// Obsolete
pub const LC_LOADFVMLIB: u32 = 0x6;
/// Obsolete
pub const LC_IDFVMLIB: u32 = 0x7;
/// Obsolete
pub const LC_IDENT: u32 = 0x8;

pub const LC_FVMFILE: u32 = 0x9;

/// No information
pub const LC_PREPAGE: u32 = 0xa;

pub const LC_DYSYMTAB: u32 = 0xb;
pub const LC_LOAD_DYLIB: u32 = 0xc;
pub const LC_ID_DYLIB: u32 = 0xd;
pub const LC_LOAD_DYLINKER: u32 = 0xe;
pub const LC_ID_DYLINKER: u32 = 0xf;
pub const LC_PREBOUND_DYLIB: u32 = 0x10;
pub const LC_ROUTINES: u32 = 0x11;
pub const LC_SUB_FRAMEWORK: u32 = 0x12;
pub const LC_SUB_UMBRELLA: u32 = 0x13;
pub const LC_SUB_CLIENT: u32 = 0x14;
pub const LC_SUB_LIBRARY: u32 = 0x15;
pub const LC_TWOLEVEL_HINTS: u32 = 0x16;
pub const LC_PREBIND_CKSUM: u32 = 0x17;

pub const LC_LOAD_WEAK_DYLIB: u32 = 0x18 | LC_REQ_DYLD;

pub const LC_SEGMENT_64: u32 = 0x19;
pub const LC_ROUTINES_64: u32 = 0x1a;
pub const LC_UUID: u32 = 0x1b;
pub const LC_RPATH: u32 = 0x1c | LC_REQ_DYLD;
pub const LC_CODE_SIGNATURE: u32 = 0x1d;
pub const LC_SEGMENT_SPLIT_INFO: u32 = 0x1e;
pub const LC_REEXPORT_DYLIB: u32 = 0x1f | LC_REQ_DYLD;

/// No info
pub const LC_LAZY_LOAD_DYLIB: u32 = 0x20;

pub const LC_ENCRYPTION_INFO: u32 = 0x21;
pub const LC_DYLD_INFO: u32 = 0x22;
pub const LC_DYLD_INFO_ONLY: u32 = 0x22 | LC_REQ_DYLD;

/// No info
pub const LC_LOAD_UPWARD_DYLIB: u32 = 0x23 | LC_REQ_DYLD;

pub const LC_VERSION_MIN_MACOSX: u32 = 0x24;
pub const LC_VERSION_MIN_IPHONEOS: u32 = 0x25;
pub const LC_FUNCTION_STARTS: u32 = 0x26;
pub const LC_DYLD_ENVIRONMENT: u32 = 0x27;
pub const LC_MAIN: u32 = 0x28 | LC_REQ_DYLD;
pub const LC_DATA_IN_CODE: u32 = 0x29;
pub const LC_SOURCE_VERSION: u32 = 0x2A;
pub const LC_DYLIB_CODE_SIGN_DRS: u32 = 0x2B;
pub const LC_ENCRYPTION_INFO_64: u32 = 0x2C;
pub const LC_LINKER_OPTION: u32 = 0x2D;
pub const LC_LINKER_OPTIMIZATION_HINT: u32 = 0x2E;
pub const LC_VERSION_MIN_TVOS: u32 = 0x2F;
pub const LC_VERSION_MIN_WATCHOS: u32 = 0x30;
pub const LC_NOTE: u32 = 0x31;
pub const LC_BUILD_VERSION: u32 = 0x32;

/// Represents general load command struct - `load_command`
#[derive(Debug)]
pub struct LoadCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    
    pub variant: LcVariant,
}

impl LoadCommand {
    pub(super) fn parse(
        reader: RcReader,
        base_offset: usize,
        endian: scroll::Endian,
    ) -> Result<LoadCommand> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let cmd: u32 = reader_mut.ioread_with(endian)?;
        let cmdsize: u32 = reader_mut.ioread_with(endian)?;

        std::mem::drop(reader_mut);

        let variant = LcVariant::parse(reader.clone(), cmd, endian)?;

        Ok(LoadCommand { cmd, cmdsize, variant })
    }
}

/// Load command has variable set of fields dependent to `cmd` field
/// List of load commands here - <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html>
#[derive(Debug)]
pub enum LcVariant {
    /// LC_SEGMENT
    Segment(LcSegment),
    /// LC_SEGMENT_64
    Segment64(LcSegment64),
    /// LC_ID_DYLIB
    IdDylib(LcDylib),
    /// LC_LOAD_DYLIB
    LoadDylib(LcDylib),
    /// LC_LOAD_WEAK_DYLIB
    LoadWeakDylib(LcDylib),
    /// LC_REEXPORT_DYLIB
    ReexportDylib(LcDylib),
    /// LC_SUB_FRAMEWORK
    Subframework(LcSubframework),
    /// LC_SUB_CLIENT
    Subclient(LcSubclient),
    /// LC_SUB_UMBRELLA
    Subumbrella(LcSubumbrella),
    /// LC_SUB_LIBRARY
    Sublibrary(LcSublibrary),
    /// LC_PREBOUND_DYLIB
    PreboundDylib(LcPreboundDylib),
    /// LC_ID_DYLINKER, 
    IdDylinker(LcDylinker),
    /// LC_LOAD_DYLINKER, 
    LoadDylinker(LcDylinker),
    /// LC_DYLD_ENVIRONMENT
    DyldEnvironment(LcDylinker),
    /// LC_THREAD
    Thread(LcThread),
    /// LC_UNIXTHREAD
    UnixThread(LcThread),
    /// LC_ROUTINES
    Routines(LcRoutines),
    /// LC_ROUTINES_64
    Routines64(LcRoutines64),
    /// LC_SYMTAB
    Symtab(LcSymtab),
    /// LC_DYSYMTAB
    Dysimtab(LcDysimtab),
    /// LC_TWOLEVEL_HINTS
    TwoLevelHints(LcTwoLevelHints),
    /// LC_PREBIND_CKSUM
    PrebindChekSum(LcPrebindChekSum),
    /// LC_UUID
    Uuid(LcUuid),
    /// LC_RPATH
    Rpath(LcRpath),
    /// LC_CODE_SIGNATURE,
    CodeSignature(LcLinkEditData),
    /// LC_SEGMENT_SPLIT_INFO,
    SegmentSplitInfo(LcLinkEditData),
    /// LC_FUNCTION_STARTS,
    FunctionStarts(LcLinkEditData),
    /// LC_DATA_IN_CODE,
    DataInCode(LcLinkEditData),
    /// LC_DYLIB_CODE_SIGN_DRS,
    DylibCodeSignature(LcLinkEditData),
    /// LC_LINKER_OPTIMIZATION_HINT,
    LinkerOptimizationHint(LcLinkEditData),
    /// LC_ENCRYPTION_INFO
    EncryptionInfo(LcEncryptionInfo),
    /// LC_ENCRYPTION_INFO_64
    EncryptionInfo64(LcEncryptionInfo64),
    /// LC_VERSION_MIN_MACOSX,
    VersionMinMacOsx(LcVersionMin),
    /// LC_VERSION_MIN_IPHONEOS,
    VersionMinIphoneOs(LcVersionMin),
    /// LC_VERSION_MIN_WATCHOS,
    VersionMinWatchOs(LcVersionMin),
    /// LC_VERSION_MIN_TVOS,
    VersionMinTvOs(LcVersionMin),
    /// LC_BUILD_VERSION
    BuildVersion(LcBuildVersion),
    /// LC_DYLD_INFO
    DyldInfo(LcDyldInfo),
    /// LC_DYLD_INFO_ONLY
    DyldInfoOnly(LcDyldInfo),
    /// LC_LINKER_OPTION
    LinkerOption(LcLinkerOption),
    /// LC_SYMSEG
    SymSeg(LcSymSeg),
    /// LC_FVMFILE
    FvmFile(LcFvmFile),
    /// LC_MAIN
    EntryPoint(LcEntryPoint),
    /// LC_SOURCE_VERSION
    SourceVersion(LcSourceVersion),
    /// LC_NOTE
    Note(LcNote),
    /// Any other command type unknown for lib
    Other,
}

impl LcVariant {
    fn parse(reader: RcReader, cmd: u32, endian: Endian) -> Result<Self> {
        let mut reader_mut = reader.borrow_mut();
        // We assume reader already stay right after `cmd` and `cmdsize`
        match cmd {
            LC_SEGMENT => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Segment(c))
            },
            LC_SEGMENT_64 => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Segment64(c))
            },
            LC_ID_DYLIB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::IdDylib(c))
            },
            LC_LOAD_DYLIB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LoadDylib(c))
            },
            LC_LOAD_WEAK_DYLIB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LoadWeakDylib(c))
            },
            LC_REEXPORT_DYLIB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::ReexportDylib(c))
            },
            LC_SUB_FRAMEWORK => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Subframework(c))
            },
            LC_SUB_CLIENT => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Subclient(c))
            },
            LC_SUB_UMBRELLA => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Subumbrella(c))
            },
            LC_SUB_LIBRARY => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Sublibrary(c))
            },
            LC_PREBOUND_DYLIB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::PreboundDylib(c))
            },
            LC_ID_DYLINKER => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::IdDylinker(c))
            },
            LC_LOAD_DYLINKER => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LoadDylinker(c))
            },
            LC_DYLD_ENVIRONMENT => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DyldEnvironment(c))
            },
            LC_THREAD => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Thread(c))
            },
            LC_UNIXTHREAD => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Thread(c))
            },
            LC_ROUTINES => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Routines(c))
            },
            LC_ROUTINES_64 => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Routines64(c))
            },
            LC_SYMTAB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Symtab(c))
            },
            LC_DYSYMTAB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Dysimtab(c))
            },
            LC_TWOLEVEL_HINTS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::TwoLevelHints(c))
            },
            LC_PREBIND_CKSUM => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::PrebindChekSum(c))
            },
            LC_UUID => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Uuid(c))
            },
            LC_RPATH => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Rpath(c))
            },
            LC_CODE_SIGNATURE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::CodeSignature(c))
            },
            LC_SEGMENT_SPLIT_INFO => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::SegmentSplitInfo(c))
            },
            LC_FUNCTION_STARTS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::FunctionStarts(c))
            },
            LC_DATA_IN_CODE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DataInCode(c))
            },
            LC_DYLIB_CODE_SIGN_DRS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DylibCodeSignature(c))
            },
            LC_LINKER_OPTIMIZATION_HINT => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LinkerOptimizationHint(c))
            },
            LC_ENCRYPTION_INFO => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::EncryptionInfo(c))
            },
            LC_ENCRYPTION_INFO_64 => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::EncryptionInfo64(c))
            },
            LC_VERSION_MIN_MACOSX => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinMacOsx(c))
            },
            LC_VERSION_MIN_IPHONEOS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinIphoneOs(c))
            },
            LC_VERSION_MIN_WATCHOS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinWatchOs(c))
            },
            LC_VERSION_MIN_TVOS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinTvOs(c))
            },
            LC_BUILD_VERSION => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::BuildVersion(c))
            },
            LC_DYLD_INFO => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DyldInfo(c))
            },
            LC_DYLD_INFO_ONLY => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DyldInfoOnly(c))
            },
            LC_LINKER_OPTION => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LinkerOption(c))
            },
            LC_SYMSEG => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::SymSeg(c))
            },
            LC_FVMFILE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::FvmFile(c))
            },
            LC_MAIN => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::EntryPoint(c))
            },
            LC_SOURCE_VERSION => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::SourceVersion(c))
            },
            LC_NOTE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Note(c))
            },
            _ => {
                Ok(Self::Other)
            },
        }
    }
}

/// `segment_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSegment {
    pub segname: [u8; 16],
    pub vmaddr: u32,
    pub vmsize: u32,
    pub fileoff: u32,
    pub filesize: u32,
    pub maxprot: VmProt,
    pub initprot: VmProt,
    pub nsects: u32,
    pub flags: u32,
}

/// `segment_command_64`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSegment64 {
    pub segname: [u8; 16],
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
    pub maxprot: VmProt,
    pub initprot: VmProt,
    pub nsects: u32,
    pub flags: u32,
}

/// LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcDylib {
    pub dylib: Dylib,
}

#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct Dylib {
    pub name: LcStr,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatibility_version: u32,
}

/// LC_SUB_FRAMEWORK
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSubframework {
    pub umbrella: LcStr,
}

/// LC_SUB_CLIENT
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSubclient {
    pub client: LcStr,
}

/// LC_SUB_UMBRELLA
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSubumbrella {
    pub sub_umbrella: LcStr,
}

/// LC_SUB_LIBRARY
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSublibrary {
    pub sub_library: LcStr,
}

/// LC_PREBOUND_DYLIB
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcPreboundDylib {
    pub name: LcStr,
    pub nmodules: u32,
    pub linked_modules: LcStr,
}

/// LC_ID_DYLINKER, LC_LOAD_DYLINKER, LC_DYLD_ENVIRONMENT
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcDylinker {
    pub name: LcStr,
}

/// LC_THREAD or LC_UNIXTHREAD
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcThread {
    flavor: u32,
    count: u32,
	/* struct XXX_thread_state state   thread state for this flavor */
	/* ... */
}

/// `routines_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcRoutines {
    pub init_address: u32,
    pub init_module: u32,

    /*
    uint32_t	reserved1;
	uint32_t	reserved2;
	uint32_t	reserved3;
	uint32_t	reserved4;
	uint32_t	reserved5;
	uint32_t	reserved6;
    */
    pub reserved: [u32; 6],
}

/// `routines_command_64`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcRoutines64 {
    pub init_address: u32,
    pub init_module: u32,

    /*
    uint64_t	reserved1;
    uint64_t	reserved2;
    uint64_t	reserved3;
    uint64_t	reserved4;
    uint64_t	reserved5;
    uint64_t	reserved6;
    */
    pub reserved: [u64; 6],
}

/// `symtab_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSymtab {
    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,
}

/// `dysymtab_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcDysimtab {
    pub ilocalsym: u32,
    pub nlocalsym: u32,

    pub iextdefsym: u32,
    pub nextdefsym: u32,

    pub iundefsym: u32,
    pub nundefsym: u32,

    pub tocoff: u32,
    pub ntoc: u32,

    pub modtaboff: u32,
    pub nmodtab: u32,

    pub extrefsymoff: u32,
    pub nextrefsyms: u32,

    pub indirectsymoff: u32,
    pub nindirectsyms: u32,

    pub extreloff: u32,
    pub nextrel: u32,

    pub locreloff: u32,
    pub nlocrel: u32,
}

/// `twolevel_hints_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcTwoLevelHints {
    pub offset: u32,
    pub nhints: u32,
}

/// `prebind_cksum_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcPrebindChekSum {
    pub cksum: u32,
}

/// `uuid_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcUuid {
    pub uuid: [u8; 16],
}

/// `rpath_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcRpath {
    pub path: LcStr,
}

/// `linkedit_data_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcLinkEditData {
    pub dataoff: u32,
    pub datasize: u32,
}

/// `encryption_info_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcEncryptionInfo {
    pub cryptoff: u32,
    pub cryptsize: u32,
    pub cryptid: u32,
}

/// `encryption_info_command_64`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcEncryptionInfo64 {
    pub cryptoff: u32,
    pub cryptsize: u32,
    pub cryptid: u32,
    pub pad: u32,
}

/// `version_min_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcVersionMin {
    pub version: u32,
    pub sdk: u32,
}

/// `build_version_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcBuildVersion {
    pub platform: u32,
    pub minos: u32,
    pub sdk: u32,
    pub ntools: u32,

    // TODO: Accurate way to provide BuildToolVersion
    // tools: (),
}

/// `build_tool_version`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct BuildToolVersion {
    pub tool: u32,
    pub version: u32,
}

/// `dyld_info_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcDyldInfo {
    pub rebase_off: u32,
    pub rebase_size: u32,

    pub bind_off: u32,
    pub bind_size: u32,

    pub weak_bind_off: u32,
    pub weak_bind_size: u32,

    pub lazy_bind_off: u32,
    pub lazy_bind_size: u32,

    pub export_off: u32,
    pub export_size: u32,
}

/// `linker_option_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcLinkerOption {
    pub count: u32,

    // TODO: concatenation of zero terminated UTF8 strings.
    // Zero filled at end to align
    // strings: (),
}

/// `symseg_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSymSeg {
    pub offset: u32,
    pub size: u32,
}

/// `fvmfile_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcFvmFile {
    pub name: LcStr,
    pub header_addr: u32,
}

/// `entry_point_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcEntryPoint {
    pub entryoff: u64,
    pub stacksize: u64,
}

/// `source_version_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcSourceVersion {
    pub version: u64,
}

/// `note_command`
#[repr(C)]
#[derive(Debug, IOread, SizeWith)]
pub struct LcNote {
    pub data_owner: [u8; 16],
    pub offset: u64,
    pub size: u64,
}
