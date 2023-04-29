use super::constants::*;
use super::RcReader;
use super::Result;
use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

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
    /// LC_IDENT
    Ident(LcIdent),
    /// LC_FVMFILE
    FvmFile(LcFvmFile),
    /// LC_MAIN
    EntryPoint(LcEntryPoint),
    /// LC_SOURCE_VERSION
    SourceVersion(LcSourceVersion),
    /// LC_NOTE
    Note(LcNote),
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

/// ident_command (Obsolete)
pub struct LcIdent;

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
    size: u64
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
    MacOsX,   // LC_VERSION_MIN_MACOSX
    IphoneOs, // LC_VERSION_MIN_IPHONEOS
    WatchOs,  // LC_VERSION_MIN_WATCHOS
    TvOs,     // LC_VERSION_MIN_TVOS
}
