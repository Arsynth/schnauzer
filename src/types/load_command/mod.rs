use crate::X64Context;

use super::fmt_ext::*;
use super::Section;
use super::RcReader;
use super::Result;
use scroll::{Endian, IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

pub mod constants;
pub use constants::*;

pub mod common;
pub use common::*;

pub mod segment_command;
pub use segment_command::*;

pub mod symtab_command;
pub use symtab_command::*;

pub mod dylib_command;
pub use dylib_command::*;

pub mod sub_framework_command;
pub use sub_framework_command::*;

pub mod sub_client_command;
pub use sub_client_command::*;

pub mod sub_umbrella_command;
pub use sub_umbrella_command::*;

pub mod sub_library_command;
pub use sub_library_command::*;

pub mod prebound_dylib_command;
pub use prebound_dylib_command::*;

pub mod dylinker_command;
pub use dylinker_command::*;

pub mod thread_command;
pub use thread_command::*;

pub mod routines_command;
pub use routines_command::*;

pub mod dysymtab_command;
pub use dysymtab_command::*;

pub mod twolevel_hints_command;
pub use twolevel_hints_command::*;

pub mod prebind_cksum_command;
pub use prebind_cksum_command::*;

pub mod uuid_command;
pub use uuid_command::*;

pub mod rpath_command;
pub use rpath_command::*;

pub mod linkedit_data_command;
pub use linkedit_data_command::*;

pub mod encryption_info_command;
pub use encryption_info_command::*;

pub mod version_min_command;
pub use version_min_command::*;

pub mod build_version_command;
pub use build_version_command::*;

pub mod dyld_info_command;
pub use dyld_info_command::*;

pub mod linker_option_command;
pub use linker_option_command::*;

pub mod note_command;
pub use note_command::*;

pub mod source_version_command;
pub use source_version_command::*;

pub mod entry_point_command;
pub use entry_point_command::*;

pub mod fvmfile_command;
pub use fvmfile_command::*;

pub mod symseg_command;
pub use symseg_command::*;

/// Represents general load command struct - `load_command`
#[derive(AutoEnumFields)]
pub struct LoadCommand {
    pub cmd: u32,
    pub cmdsize: u32,

    pub variant: LcVariant,
}

impl Debug for LoadCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadCommand")
            .field("cmd", &load_command_to_string(self.cmd))
            .field("cmdsize", &self.cmdsize)
            .field("variant", &self.variant)
            .finish()
    }
}

impl LoadCommand {
    pub(super) fn parse(
        reader: RcReader,
        base_offset: usize,
        endian: scroll::Endian,
        is_64: bool,
        object_file_offset: u64,
    ) -> Result<LoadCommand> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let cmd: u32 = reader_mut.ioread_with(endian)?;
        let cmdsize: u32 = reader_mut.ioread_with(endian)?;

        std::mem::drop(reader_mut);

        let variant = LcVariant::parse(
            reader.clone(),
            cmd,
            cmdsize,
            base_offset,
            endian,
            is_64,
            object_file_offset,
        )?;

        Ok(LoadCommand {
            cmd,
            cmdsize,
            variant,
        })
    }
}

/// Load command has variable set of fields dependent to `cmd` field
/// List of load commands here - <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html>
#[derive(Debug, AutoEnumFields)]
pub enum LcVariant {
    /// LC_SEGMENT
    Segment32(LcSegment),
    /// LC_SEGMENT_64
    Segment64(LcSegment),
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
    fn parse(
        reader: RcReader,
        cmd: u32,
        cmdsize: u32,
        command_offset: usize,
        endian: Endian,
        is_64: bool,
        object_file_offset: u64,
    ) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();
        let base_offset = reader_mut.stream_position()? as usize;
        // We assume reader already stay right after `cmd` and `cmdsize`
        match cmd {
            LC_SEGMENT => {
                std::mem::drop(reader_mut);
                let c = LcSegment::parse(reader_clone, base_offset, object_file_offset, X64Context::Off(endian))?;
                Ok(Self::Segment32(c))
            }
            LC_SEGMENT_64 => {
                std::mem::drop(reader_mut);
                let c = LcSegment::parse(reader_clone, base_offset, object_file_offset, X64Context::On(endian))?;
                Ok(Self::Segment64(c))
            }
            LC_ID_DYLIB => {
                std::mem::drop(reader_mut);
                let c = LcDylib::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::IdDylib(c))
            }
            LC_LOAD_DYLIB => {
                std::mem::drop(reader_mut);
                let c = LcDylib::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::LoadDylib(c))
            }
            LC_LOAD_WEAK_DYLIB => {
                std::mem::drop(reader_mut);
                let c = LcDylib::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::LoadWeakDylib(c))
            }
            LC_REEXPORT_DYLIB => {
                std::mem::drop(reader_mut);
                let c = LcDylib::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::ReexportDylib(c))
            }
            LC_SUB_FRAMEWORK => {
                std::mem::drop(reader_mut);
                let c = LcSubframework::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::Subframework(c))
            }
            LC_SUB_CLIENT => {
                std::mem::drop(reader_mut);
                let c = LcSubclient::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::Subclient(c))
            }
            LC_SUB_UMBRELLA => {
                std::mem::drop(reader_mut);
                let c = LcSubumbrella::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::Subumbrella(c))
            }
            LC_SUB_LIBRARY => {
                std::mem::drop(reader_mut);
                let c = LcSublibrary::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::Sublibrary(c))
            }
            LC_PREBOUND_DYLIB => {
                std::mem::drop(reader_mut);
                let c = LcPreboundDylib::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::PreboundDylib(c))
            }
            LC_ID_DYLINKER => {
                std::mem::drop(reader_mut);
                let c = LcDylinker::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::IdDylinker(c))
            }
            LC_LOAD_DYLINKER => {
                std::mem::drop(reader_mut);
                let c = LcDylinker::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::LoadDylinker(c))
            }
            LC_DYLD_ENVIRONMENT => {
                std::mem::drop(reader_mut);
                let c = LcDylinker::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::DyldEnvironment(c))
            }
            LC_THREAD => {
                std::mem::drop(reader_mut);
                let c = LcThread::parse(reader, cmdsize, base_offset, endian)?;
                Ok(Self::Thread(c))
            }
            LC_UNIXTHREAD => {
                std::mem::drop(reader_mut);
                let c = LcThread::parse(reader, cmdsize, base_offset, endian)?;
                Ok(Self::Thread(c))
            }
            LC_ROUTINES => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Routines(c))
            }
            LC_ROUTINES_64 => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Routines64(c))
            }
            LC_SYMTAB => {
                std::mem::drop(reader_mut);
                let c =
                    LcSymtab::parse(reader_clone, is_64, base_offset, endian, object_file_offset)?;
                Ok(Self::Symtab(c))
            }
            LC_DYSYMTAB => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Dysimtab(c))
            }
            LC_TWOLEVEL_HINTS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::TwoLevelHints(c))
            }
            LC_PREBIND_CKSUM => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::PrebindChekSum(c))
            }
            LC_UUID => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Uuid(c))
            }
            LC_RPATH => {
                std::mem::drop(reader_mut);
                let c = LcRpath::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::Rpath(c))
            }
            LC_CODE_SIGNATURE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::CodeSignature(c))
            }
            LC_SEGMENT_SPLIT_INFO => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::SegmentSplitInfo(c))
            }
            LC_FUNCTION_STARTS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::FunctionStarts(c))
            }
            LC_DATA_IN_CODE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DataInCode(c))
            }
            LC_DYLIB_CODE_SIGN_DRS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DylibCodeSignature(c))
            }
            LC_LINKER_OPTIMIZATION_HINT => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LinkerOptimizationHint(c))
            }
            LC_ENCRYPTION_INFO => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::EncryptionInfo(c))
            }
            LC_ENCRYPTION_INFO_64 => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::EncryptionInfo64(c))
            }
            LC_VERSION_MIN_MACOSX => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinMacOsx(c))
            }
            LC_VERSION_MIN_IPHONEOS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinIphoneOs(c))
            }
            LC_VERSION_MIN_WATCHOS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinWatchOs(c))
            }
            LC_VERSION_MIN_TVOS => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::VersionMinTvOs(c))
            }
            LC_BUILD_VERSION => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::BuildVersion(c))
            }
            LC_DYLD_INFO => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DyldInfo(c))
            }
            LC_DYLD_INFO_ONLY => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::DyldInfoOnly(c))
            }
            LC_LINKER_OPTION => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::LinkerOption(c))
            }
            LC_SYMSEG => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::SymSeg(c))
            }
            LC_FVMFILE => {
                std::mem::drop(reader_mut);
                let c = LcFvmFile::parse(reader_clone, command_offset, base_offset, endian)?;
                Ok(Self::FvmFile(c))
            }
            LC_MAIN => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::EntryPoint(c))
            }
            LC_SOURCE_VERSION => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::SourceVersion(c))
            }
            LC_NOTE => {
                let c = reader_mut.ioread_with(endian)?;
                Ok(Self::Note(c))
            }
            _ => Ok(Self::Other),
        }
    }
}
