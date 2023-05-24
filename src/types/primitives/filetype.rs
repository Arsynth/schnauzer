//! <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html>
//! The layout of the file depends on the filetype.  For all but the MH_OBJECT
//! file type the segments are padded out and aligned on a segment alignment
//! boundary for efficient demand pageing.  The MH_EXECUTE, MH_FVMLIB, MH_DYLIB,
//! MH_DYLINKER and MH_BUNDLE file types also have the headers included as part
//! of their first segment.
//!
//! The file type MH_OBJECT is a compact format intended as output of the
//! assembler and input (and possibly output) of the link editor (the .o
//! format).  All sections are in one unnamed segment with no segment padding.
//! This format is used as an executable format when the file is so small the
//! segment padding greatly increases its size.
//!
//! The file type MH_PRELOAD is an executable format intended for things that
//! are not executed under the kernel (proms, stand alones, kernels, etc).  The
//! format can be executed under the kernel but may demand paged it and not
//! preload it before execution.
//!
//! A core file is in MH_CORE format and can be any in an arbritray legal
//! Mach-O file.
//!
//! Constants for the filetype field of the mach_header

use std::fmt::Debug;
use std::fmt::Display;

use scroll::{IOread, SizeWith};

pub mod filetype_constants {
    /// relocatable object file  
    pub const MH_OBJECT: u32 = 0x1;
    /// demand paged executable file  
    pub const MH_EXECUTE: u32 = 0x2;
    /// fixed VM shared library file   
    pub const MH_FVMLIB: u32 = 0x3;
    /// core file  
    pub const MH_CORE: u32 = 0x4;
    /// preloaded executable file  
    pub const MH_PRELOAD: u32 = 0x5;
    /// dynamically bound shared library  
    pub const MH_DYLIB: u32 = 0x6;
    /// dynamic link editor  
    pub const MH_DYLINKER: u32 = 0x7;
    /// dynamically bound bundle file  
    pub const MH_BUNDLE: u32 = 0x8;
    /// shared library stub for static linking only, no section contents  
    pub const MH_DYLIB_STUB: u32 = 0x9;
    /// companion file with only debug sections  
    pub const MH_DSYM: u32 = 0xa;
    /// x86_64 kexts  
    pub const MH_KEXT_BUNDLE: u32 = 0xb;
}

use self::filetype_constants::*;

#[derive(IOread, SizeWith)]
pub struct FileType(pub u32);

impl FileType {
    pub fn string_value(&self) -> String {
        match self.0 {
            MH_OBJECT => "Object".to_string(),
            MH_EXECUTE => "Exec".to_string(),
            MH_FVMLIB => "FVMLib".to_string(),
            MH_CORE => "Core".to_string(),
            MH_PRELOAD => "Preloaded".to_string(),
            MH_DYLIB => "Dylib".to_string(),
            MH_DYLINKER => "Dylinker".to_string(),
            MH_BUNDLE => "Bundle".to_string(),
            MH_DYLIB_STUB => "Dylib stub".to_string(),
            MH_DSYM => "dSYM".to_string(),
            MH_KEXT_BUNDLE => "Kext bundle".to_string(),
            any => any.to_string(),
        }
    }
}

impl Debug for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_value())
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string_value())
    }
}
