use super::auto_enum_fields::*;
use super::primitives::*;
use super::Magic;
use super::RcReader;
use super::Result;
use schnauzer_derive::AutoEnumFields;
use scroll::IOread;

use std::fmt::Debug;

pub mod flags {
    /// The object file has no undefined references
    pub const MH_NOUNDEFS: u32 = 0x1;
    /// the object file is the output of an incremental link against a base file and can't be link edited again  
    pub const MH_INCRLINK: u32 = 0x2;
    /// the object file is input for the dynamic linker and can't be staticly link edited again  
    pub const MH_DYLDLINK: u32 = 0x4;
    /// the object file's undefined references are bound by the dynamic linker when loaded.  
    pub const MH_BINDATLOAD: u32 = 0x8;
    /// the file has its dynamic undefined references prebound.   
    pub const MH_PREBOUND: u32 = 0x10;
    /// the file has its read-only and read-write segments split  
    pub const MH_SPLIT_SEGS: u32 = 0x20;
    /// the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)   
    pub const MH_LAZY_INIT: u32 = 0x40;
    /// the image is using two-level name space bindings   
    pub const MH_TWOLEVEL: u32 = 0x80;
    /// the executable is forcing all images to use flat name space bindings  
    pub const MH_FORCE_FLAT: u32 = 0x100;
    /// this umbrella guarantees no multiple defintions of symbols in its sub-images so the two-level namespace hints can always be used.  
    pub const MH_NOMULTIDEFS: u32 = 0x200;
    /// do not have dyld notify the prebinding agent about this executable  
    pub const MH_NOFIXPREBINDING: u32 = 0x400;
    /// the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.  
    pub const MH_PREBINDABLE: u32 = 0x800;
    /// indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.  
    pub const MH_ALLMODSBOUND: u32 = 0x1000;
    /// safe to divide up the sections into sub-sections via symbols for dead code stripping  
    pub const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 0x2000;
    /// the binary has been canonicalized via the unprebind operation  
    pub const MH_CANONICAL: u32 = 0x4000;
    /// the final linked image contains external weak symbols  
    pub const MH_WEAK_DEFINES: u32 = 0x8000;
    /// the final linked image uses weak symbols  
    pub const MH_BINDS_TO_WEAK: u32 = 0x10000;
    /// When this bit is set, all stacks in the task will be given stack execution privilege.  Only used in MH_EXECUTE filetypes.  
    pub const MH_ALLOW_STACK_EXECUTION: u32 = 0x20000;
    /// When this bit is set, the binary declares it is safe for use in processes with uid zero  
    pub const MH_ROOT_SAFE: u32 = 0x40000;
    /// When this bit is set, the binary declares it is safe for use in processes when issetugid() is true  
    pub const MH_SETUID_SAFE: u32 = 0x80000;
    /// When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported  
    pub const MH_NO_REEXPORTED_DYLIBS: u32 = 0x100000;
    /// When this bit is set, the OS will load the main executable at a random address.  Only used in MH_EXECUTE filetypes.  
    pub const MH_PIE: u32 = 0x200000;
    /// Only for use on dylibs.  When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib.  
    pub const MH_DEAD_STRIPPABLE_DYLIB: u32 = 0x400000;
    /// Contains a section of type S_THREAD_LOCAL_VARIABLES  
    pub const MH_HAS_TLV_DESCRIPTORS: u32 = 0x800000;
    /// When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.  
    pub const MH_NO_HEAP_EXECUTION: u32 = 0x1000000;
    /// The code was linked for use in an application extension.  
    pub const MH_APP_EXTENSION_SAFE: u32 = 0x02000000;
}

#[derive(AutoEnumFields)]
pub struct MachHeader {
    pub magic: Magic,
    pub cputype: CPUType,
    pub cpusubtype: CPUSubtype,
    pub filetype: FileType,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: Hu32,
    pub reserved: Hu32, // For 64 bit headers
}

impl MachHeader {
    /// We assume reader is already stands on correct position
    pub(super) fn parse(reader: RcReader) -> Result<MachHeader> {
        let mut reader_mut = reader.borrow_mut();

        let mut ctx = scroll::BE;

        let magic: u32 = reader_mut.ioread_with(ctx)?;
        let magic: Magic = magic.try_into()?;

        if magic.is_reverse() {
            ctx = scroll::LE;
        }
        let ctx = ctx;

        let cpu_type: CPUType = reader_mut.ioread_with(ctx)?;
        let cpu_subtype: CPUSubtype = reader_mut.ioread_with(ctx)?;
        let file_type: FileType = reader_mut.ioread_with(ctx)?;
        let ncmds: u32 = reader_mut.ioread_with(ctx)?;
        let size_of_cmds: u32 = reader_mut.ioread_with(ctx)?;
        let flags: u32 = reader_mut.ioread_with(ctx)?;

        let mut reserved = 0u32;
        if magic.is_64() {
            reserved = reader_mut.ioread_with(ctx)?;
        }

        Ok(MachHeader {
            magic,
            cputype: cpu_type,
            cpusubtype: cpu_subtype,
            filetype: file_type,
            ncmds,
            sizeofcmds: size_of_cmds,
            flags: Hu32(flags),
            reserved: Hu32(reserved),
        })
    }
}

impl Debug for MachHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachHeader")
            .field("magic", &self.magic)
            .field("cpu_type", &self.cputype)
            .field("cpu_subtype", &self.cpusubtype)
            .field("file_type", &self.filetype)
            .field("ncmds", &self.ncmds)
            .field("size_of_cmds", &self.sizeofcmds)
            .field("flags", &self.flags)
            .field("reserved", &self.reserved)
            .finish()
    }
}
