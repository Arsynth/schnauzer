use super::constants::*;
use super::RcReader;
use super::Result;
use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

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
    PrebindChekSum(LcPrebindChekSum),     // prebind_cksum_command LC_PREBIND_CKSUM
    Uuid(LcUuid),                         // uuid_command LC_UUID
    Rpath(LcRpath),                       // rpath_command LC_RPATH
    LinkEditData(LcLinkEditData),         // linkedit_data_command LC_...(See `LinkEditDataVariant`)
    EncryptionInfo(LcEncryptionInfo),     // encryption_info_command LC_ENCRYPTION_INFO
    EncryptionInfo64(LcEncryptionInfo64), // encryption_info_command_64 LC_ENCRYPTION_INFO_64
    VersionMin(LcVersionMin), // version_min_command LC_VERSION_MIN_...(See `VersionMinVariant`)
    BuildVersion(LcBuildVersion), // build_version_command LC_BUILD_VERSION
    DyldInfo(LcDyldInfo),     // dyld_info_command LC_DYLD_INFO
    LinkerOption(LcLinkerOption), // linker_option_command LC_LINKER_OPTION
    SymSeg(LcSymSeg),         // symseg_command LC_SYMSEG
    Ident(LcIdent),           // ident_command LC_IDENT
    FvmFile(LcFvmFile),       // fvmfile_command LC_FVMFILE
    EntryPoint(LcEntryPoint), // entry_point_command LC_MAIN
    SourceVersion(LcSourceVersion), // source_version_command LC_SOURCE_VERSION
    Note(LcNote),             // note_command LC_NOTE
}

pub struct LcPrebindChekSum;
pub struct LcUuid;
pub struct LcRpath;
pub struct LcEncryptionInfo;
pub struct LcEncryptionInfo64;
pub struct LcBuildVersion;
pub struct LcDyldInfo;
pub struct LcLinkerOption;
pub struct LcSymSeg;
pub struct LcIdent;
pub struct LcFvmFile;
pub struct LcEntryPoint;
pub struct LcSourceVersion;
pub struct LcNote;

pub struct LcLinkEditData {
    variant: LinkEditDataVariant
}
pub enum LinkEditDataVariant {
    CodeSignature, // LC_CODE_SIGNATURE
    SegmentSplitInfo, // LC_SEGMENT_SPLIT_INFO
    FunctionStarts, // LC_FUNCTION_STARTS
    DataInCode, // LC_DATA_IN_CODE
    DylibCodeSignDrs, // LC_DYLIB_CODE_SIGN_DRS
    LinkerOptimizationHint, // LC_LINKER_OPTIMIZATION_HINT
}

pub struct LcVersionMin {
    variant: VersionMinVariant,
}
pub enum VersionMinVariant {
    MacOsX,   // LC_VERSION_MIN_MACOSX
    IphoneOs, // LC_VERSION_MIN_IPHONEOS
    WatchOs,  // LC_VERSION_MIN_WATCHOS
    TvOs,     // LC_VERSION_MIN_TVOS
}
