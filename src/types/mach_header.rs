use super::auto_enum_fields::*;
use super::primitives::*;
use super::Magic;
use super::RcReader;
use super::Result;
use schnauzer_derive::AutoEnumFields;
use scroll::IOread;

use std::fmt::Debug;

#[derive(AutoEnumFields)]
pub struct MachHeader {
    pub magic: Magic,
    pub cputype: CPUType,
    pub cpusubtype: CPUSubtype,
    pub filetype: FileType,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: ObjectFlags,
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
        let flags: ObjectFlags = reader_mut.ioread_with(ctx)?;

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
            flags,
            reserved: Hu32(reserved),
        })
    }
}

impl MachHeader {
    pub fn machine(&self) -> Machine {
        Machine::new(self.cputype, self.cpusubtype)
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
