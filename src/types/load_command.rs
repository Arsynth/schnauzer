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
    /// prebind_cksum_command LC_PREBIND_CKSUM
    PrebindChekSum(LcPrebindChekSum),
    /// uuid_command LC_UUID
    Uuid(LcUuid),
    /// rpath_command LC_RPATH
    Rpath(LcRpath),
    /// linkedit_data_command LC_...(See `LinkEditDataVariant`)
    LinkEditData(LcLinkEditData),
    /// encryption_info_command LC_ENCRYPTION_INFO
    EncryptionInfo(LcEncryptionInfo),
    /// encryption_info_command_64 LC_ENCRYPTION_INFO_64
    EncryptionInfo64(LcEncryptionInfo64),
    /// version_min_command LC_VERSION_MIN_...(See `VersionMinVariant`)
    VersionMin(LcVersionMin),
    /// build_version_command LC_BUILD_VERSION
    BuildVersion(LcBuildVersion),
    /// dyld_info_command LC_DYLD_INFO
    DyldInfo(LcDyldInfo),
    /// linker_option_command LC_LINKER_OPTION
    LinkerOption(LcLinkerOption),
    /// symseg_command LC_SYMSEG
    SymSeg(LcSymSeg),
    /// ident_command LC_IDENT
    Ident(LcIdent),
    /// fvmfile_command LC_FVMFILE
    FvmFile(LcFvmFile),
    /// entry_point_command LC_MAIN
    EntryPoint(LcEntryPoint),
    /// source_version_command LC_SOURCE_VERSION
    SourceVersion(LcSourceVersion),
    /// note_command LC_NOTE
    Note(LcNote),
}

pub struct LcPrebindChekSum {
    /// cksum
    check_sum: u32,
}
pub struct LcUuid {
    /// uuid[16]
    uuid: u128,
}
pub struct LcRpath {
    /// path
    path: String,
}
pub struct LcLinkEditData {
    variant: LinkEditDataVariant,
    /// dataoff
    data_offset: u32,
    /// datasize
    data_size: u32,
}
pub struct LcEncryptionInfo {
    /// cryptoff
    crypt_offset: u32,
    /// cryptsize
    crypt_size: u32,
    /// cryptid
    crypt_id: u32,
}
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
pub struct LcVersionMin {
    variant: VersionMinVariant,
    /// version
    version: u32,
    /// sdk
    sdk: u32,
}

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
    _tools: ()
}

impl LcBuildVersion {
    pub fn tool_version_iterator(&self) -> BuildToolVersionIterator {
        BuildToolVersionIterator
    }
}

pub struct BuildToolVersionIterator;
pub struct BuildToolVersion;

pub struct LcDyldInfo {
    
}
pub struct LcLinkerOption;
pub struct LcSymSeg;
pub struct LcIdent;
pub struct LcFvmFile;
pub struct LcEntryPoint;
pub struct LcSourceVersion;
pub struct LcNote;

pub enum LinkEditDataVariant {
    CodeSignature,          // LC_CODE_SIGNATURE
    SegmentSplitInfo,       // LC_SEGMENT_SPLIT_INFO
    FunctionStarts,         // LC_FUNCTION_STARTS
    DataInCode,             // LC_DATA_IN_CODE
    DylibCodeSignDrs,       // LC_DYLIB_CODE_SIGN_DRS
    LinkerOptimizationHint, // LC_LINKER_OPTIMIZATION_HINT
}

pub enum VersionMinVariant {
    MacOsX,   // LC_VERSION_MIN_MACOSX
    IphoneOs, // LC_VERSION_MIN_IPHONEOS
    WatchOs,  // LC_VERSION_MIN_WATCHOS
    TvOs,     // LC_VERSION_MIN_TVOS
}
