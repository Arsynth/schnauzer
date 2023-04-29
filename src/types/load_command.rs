use super::constants::*;
use super::RcReader;
use super::Result;
use crate::result::Error;
use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

pub const LC_REQ_DYLD: u32 = 0x80000000;

pub const LC_SEGMENT: u32 = 0x1;
pub const LC_SYMTAB: u32 = 0x2;
pub const LC_SYMSEG: u32 = 0x3;
pub const LC_THREAD: u32 = 0x4;
pub const LC_UNIXTHREAD: u32 = 0x5;
pub const LC_LOADFVMLIB: u32 = 0x6;
pub const LC_IDFVMLIB: u32 = 0x7;
pub const LC_IDENT: u32 = 0x8;
pub const LC_FVMFILE: u32 = 0x9;
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
pub const LC_LAZY_LOAD_DYLIB: u32 = 0x20;
pub const LC_ENCRYPTION_INFO: u32 = 0x21;
pub const LC_DYLD_INFO: u32 = 0x22;
pub const LC_DYLD_INFO_ONLY: u32 = 0x22 | LC_REQ_DYLD;
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
    pub(super) cmd: u32,
    pub(super) cmd_size: u32,
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
        let cmd_size: u32 = reader_mut.ioread_with(endian)?;

        Ok(LoadCommand { cmd, cmd_size })
    }
}

impl LoadCommand {
    pub fn cmd(&self) -> LoadCommandType {
        self.cmd
    }

    pub fn cmd_size(&self) -> u32 {
        self.cmd_size
    }
}

/// Load command has variable set of fields dependent to `cmd` field
/// List of load commands here - <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html>
pub enum LcVariant {
    /// LC_SEGMENT
    Segment(LcSegment),
    /// LC_SEGMENT_64
    Segment64(LcSegment64),
    /// LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB
    Dylib(LcDylib),
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
    /// LC_ID_DYLINKER, LC_LOAD_DYLINKER, LC_DYLD_ENVIRONMENT
    Dylinker(LcDylinker),
    /// LC_DYLD_ENVIRONMENT or LC_UNIXTHREAD
    Thread(LcThread),
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
    /// LC_...(See `LinkEditDataVariant`)
    LinkEditData(LcLinkEditData),
    /// LC_ENCRYPTION_INFO
    EncryptionInfo(LcEncryptionInfo),
    /// LC_ENCRYPTION_INFO_64
    EncryptionInfo64(LcEncryptionInfo64),
    /// LC_VERSION_MIN_...(See `VersionMinVariant`)
    VersionMin(LcVersionMin),
    /// LC_BUILD_VERSION
    BuildVersion(LcBuildVersion),
    /// LC_DYLD_INFO
    DyldInfo(LcDyldInfo),
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
    /// Any unknown command type
    Unknown(u32),
}
/*
impl TryFrom<u32> for LcVariant {
    type Error = Error;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            LC_SEGMENT => Ok(Self::),
            LC_SYMTAB => Ok(Self::TYPE),
            LC_SYMSEG => Ok(Self::TYPE),
            LC_THREAD => Ok(Self::TYPE),
            LC_UNIXTHREAD => Ok(Self::TYPE),
            LC_LOADFVMLIB => Ok(Self::TYPE),
            LC_IDFVMLIB => Ok(Self::TYPE),
            LC_IDENT => Ok(Self::TYPE),
            LC_FVMFILE => Ok(Self::TYPE),
            LC_PREPAGE => Ok(Self::TYPE),
            LC_DYSYMTAB => Ok(Self::TYPE),
            LC_LOAD_DYLIB => Ok(Self::TYPE),
            LC_ID_DYLIB => Ok(Self::TYPE),
            LC_LOAD_DYLINKER => Ok(Self::TYPE),
            LC_ID_DYLINKER => Ok(Self::TYPE),
            LC_PREBOUND_DYLIB => Ok(Self::TYPE),
            LC_ROUTINES => Ok(Self::TYPE),
            LC_SUB_FRAMEWORK => Ok(Self::TYPE),
            LC_SUB_UMBRELLA => Ok(Self::TYPE),
            LC_SUB_CLIENT => Ok(Self::TYPE),
            LC_SUB_LIBRARY => Ok(Self::TYPE),
            LC_TWOLEVEL_HINTS => Ok(Self::TYPE),
            LC_PREBIND_CKSUM => Ok(Self::TYPE),
            LC_LOAD_WEAK_DYLIB => Ok(Self::TYPE),
            LC_SEGMENT_64 => Ok(Self::TYPE),
            LC_ROUTINES_64 => Ok(Self::TYPE),
            LC_UUID => Ok(Self::TYPE),
            LC_RPATH => Ok(Self::TYPE),
            LC_CODE_SIGNATURE => Ok(Self::TYPE),
            LC_SEGMENT_SPLIT_INFO => Ok(Self::TYPE),
            LC_REEXPORT_DYLIB => Ok(Self::TYPE),
            LC_LAZY_LOAD_DYLIB => Ok(Self::TYPE),
            LC_ENCRYPTION_INFO => Ok(Self::TYPE),
            LC_DYLD_INFO => Ok(Self::TYPE),
            LC_DYLD_INFO_ONLY => Ok(Self::TYPE),
            LC_LOAD_UPWARD_DYLIB => Ok(Self::TYPE),
            LC_VERSION_MIN_MACOSX => Ok(Self::TYPE),
            LC_VERSION_MIN_IPHONEOS => Ok(Self::TYPE),
            LC_FUNCTION_STARTS => Ok(Self::TYPE),
            LC_DYLD_ENVIRONMENT => Ok(Self::TYPE),
            LC_MAIN => Ok(Self::TYPE),
            LC_DATA_IN_CODE => Ok(Self::TYPE),
            LC_SOURCE_VERSION => Ok(Self::TYPE),
            LC_DYLIB_CODE_SIGN_DRS => Ok(Self::TYPE),
            LC_ENCRYPTION_INFO_64 => Ok(Self::TYPE),
            LC_LINKER_OPTION => Ok(Self::TYPE),
            LC_LINKER_OPTIMIZATION_HINT => Ok(Self::TYPE),
            LC_VERSION_MIN_TVOS => Ok(Self::TYPE),
            LC_VERSION_MIN_WATCHOS => Ok(Self::TYPE),
            LC_NOTE => Ok(Self::TYPE),
            LC_BUILD_VERSION => Ok(Self::TYPE),
            _ => Ok(Self::Unknown(value)),
        }
    }
}
*/

/// LC_SEGMENT
pub struct LcSegment {
    segname: String,
    vmaddr: u32,
	vmsize: u32,
	fileoff: u32,
	filesize: u32,
	maxprot: VmProt,
	initprot: VmProt,
	nsects: u32,
	flags: u32,
}

/// LC_SEGMENT_64
pub struct LcSegment64 {
    segname: String,
    vmaddr: u64,
	vmsize: u64,
	fileoff: u64,
	filesize: u64,
	maxprot: VmProt,
	initprot: VmProt,
	nsects: u32,
	flags: u32,
}

/// LC_ID_DYLIB, LC_LOAD_{,WEAK_}DYLIB, LC_REEXPORT_DYLIB
pub struct LcDylib {
    variant: DylibVariant,
    dylib: Dylib, 
}
pub enum DylibVariant {
    /// LC_ID_DYLIB
    Id,
    /// LC_LOAD_DYLIB
    Load,
    /// LC_LOAD_WEAK_DYLIB
    LoadWeak,
    /// LC_REEXPORT_DYLIB
    Reexport
}
pub struct Dylib {
    name: String,
    timestamp: u32,
    current_version: u32,
    compatibility_version: u32,
}

/// LC_SUB_FRAMEWORK
pub struct LcSubframework {
    /// union lc_str umbrella
    umbrella: String,
}

/// LC_SUB_CLIENT
pub struct LcSubclient {
    /// union lc_str umbrella
    client: String,
}

/// LC_SUB_UMBRELLA
pub struct LcSubumbrella {
    /// union lc_str umbrella
    sub_umbrella: String,
}

/// LC_SUB_LIBRARY
pub struct LcSublibrary {
    /// union lc_str umbrella
    sub_library: String,
}

/// LC_PREBOUND_DYLIB
pub struct LcPreboundDylib {
    /// union lc_str
    name: String,
	nmodules: u32,
    /// union lc_str
	linked_modules: Vec<u8>,
}

/// LC_ID_DYLINKER, LC_LOAD_DYLINKER, LC_DYLD_ENVIRONMENT
pub struct LcDylinker {
    /// union lc_str    name;
    name: String,
}

/// LC_DYLD_ENVIRONMENT or LC_UNIXTHREAD
pub struct LcThread {
    _machine_specific: (),
}

/// LC_ROUTINES
pub struct LcRoutines {
    init_address: u32,
	init_module: u32,

    /// Would be [u32; 6]
    _reserved: (),
}

/// LC_ROUTINES_64
pub struct LcRoutines64 {
    init_address: u32,
	init_module: u32,

    /// Would be [u64; 6]
    _reserved: (),
}

/// LC_SYMTAB
pub struct LcSymtab {
    symoff: u32,
	nsyms: u32,
	stroff: u32,
	strsize: u32,
}

/// LC_DYSYMTAB
pub struct LcDysimtab {
    ilocalsym: u32,
    nlocalsym: u32,

    iextdefsym: u32,
    nextdefsym: u32,

    iundefsym: u32,
    nundefsym: u32,

    tocoff: u32,
    ntoc: u32,

    modtaboff: u32,
    nmodtab: u32,

    extrefsymoff: u32,
    nextrefsyms: u32,

    indirectsymoff: u32,
    nindirectsyms: u32,

    extreloff: u32,
    nextrel: u32,

    locreloff: u32,
    nlocrel: u32,
}

pub struct LcTwoLevelHints {
    offset: u32,
    nhints: u32,
}

/// prebind_cksum_command
pub struct LcPrebindChekSum {
    /// cksum
    check_sum: u32,
}

/// uuid_command
pub struct LcUuid {
    /// uuid[16]
    uuid: u128,
}

/// rpath_command
pub struct LcRpath {
    /// path
    path: String,
}

/// linkedit_data_command
pub struct LcLinkEditData {
    variant: LinkEditDataVariant,
    /// dataoff
    data_offset: u32,
    /// datasize
    data_size: u32,
}

/// encryption_info_command
pub struct LcEncryptionInfo {
    /// cryptoff
    crypt_offset: u32,
    /// cryptsize
    crypt_size: u32,
    /// cryptid
    crypt_id: u32,
}

/// encryption_info_command_64
pub struct LcEncryptionInfo64 {
    /// cryptoff
    crypt_offset: u32,
    /// cryptsize
    crypt_size: u32,
    /// cryptid
    crypt_id: u32,

    /// pad
    pad: u32,
}

/// version_min_command
pub struct LcVersionMin {
    variant: VersionMinVariant,
    /// version
    version: u32,
    /// sdk
    sdk: u32,
}

/// build_version_command
pub struct LcBuildVersion {
    /// platform
    platform: u32,
    /// minos
    min_os: u32,
    /// sdk
    sdk: u32,
    /// ntools
    ntools: u32,

    /// Did not loaded directly, use `tool_version_iterator()` instead
    _tools: (),
}

impl LcBuildVersion {
    pub fn tool_version_iterator(&self) -> BuildToolVersionIterator {
        BuildToolVersionIterator
    }
}

pub struct BuildToolVersionIterator;
pub struct BuildToolVersion;

/// dyld_info_command
pub struct LcDyldInfo {
    /// rebase_off
    rebase_offset: u32,
    /// rebase_size
    rebase_size: u32,

    /// bind_off
    bind_offset: u32,
    /// bind_size
    bind_size: u32,

    /// weak_bind_off
    weak_bind_offset: u32,
    /// weak_bind_size
    weak_bind_size: u32,

    /// lazy_bind_off
    lazy_bind_offset: u32,
    /// lazy_bind_size
    lazy_bind_size: u32,

    /// export_off
    export_offset: u32,
    /// export_size
    export_size: u32,
}

/// linker_option_command
pub struct LcLinkerOption {
    /// count
    count: u32,

    /// TODO: concatenation of zero terminated UTF8 strings.
    /// Zero filled at end to align
    _strings: (),
}

/// symseg_command
pub struct LcSymSeg {
    /// offset
    offset: u32,
    /// size
    size: u32,
}

/// fvmfile_command
pub struct LcFvmFile {
    /// name
    name: String,
    /// header_addr
    header_address: u32,
}

/// entry_point_command
pub struct LcEntryPoint {
    /// entryoff
    entry_offset: u64,
    /// stacksize
    stack_size: u64,
}

/// source_version_command
pub struct LcSourceVersion {
    /// version
    version: u64,
}

/// note_command
pub struct LcNote {
    /// data_owner[16]
    data_owner: String,
    /// offset
    offset: u64,
    /// size
    size: u64,
}

pub enum LinkEditDataVariant {
    /// LC_CODE_SIGNATURE
    CodeSignature,
    /// LC_SEGMENT_SPLIT_INFO
    SegmentSplitInfo,
    /// LC_FUNCTION_STARTS
    FunctionStarts,
    /// LC_DATA_IN_CODE
    DataInCode,
    /// LC_DYLIB_CODE_SIGN_DRS
    DylibCodeSignDrs,
    /// LC_LINKER_OPTIMIZATION_HINT
    LinkerOptimizationHint,
}

pub enum VersionMinVariant {
    /// LC_VERSION_MIN_MACOSX
    MacOsX,
    /// LC_VERSION_MIN_IPHONEOS
    IphoneOs,
    /// LC_VERSION_MIN_WATCHOS
    WatchOs,
    /// LC_VERSION_MIN_TVOS
    TvOs,
}
