use super::RcReader;
use scroll::IOread;
use super::constants::*;
use super::Result;
use super::Magic;

use std::fmt::{Debug};

pub struct MachHeader {
    pub(crate) magic: Magic,
    pub(crate) cpu_type: CPUType,
    pub(crate) cpu_subtype: CPUSubtype,
    pub(crate) file_type: u32,
    pub(crate) ncmds: u32,
    pub(crate) size_of_cmds: u32,
    pub(crate) flags: u32,
    pub(crate) reserved: u32, // For 64 bit headers
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
        let file_type: u32 = reader_mut.ioread_with(ctx)?;
        let ncmds: u32 = reader_mut.ioread_with(ctx)?;
        let size_of_cmds: u32 = reader_mut.ioread_with(ctx)?;
        let flags: u32 = reader_mut.ioread_with(ctx)?;

        let mut reserved = 0u32;
        if magic.is_64() {
            reserved = reader_mut.ioread_with(ctx)?;
        }

        Ok(MachHeader {
            magic,
            cpu_type,
            cpu_subtype,
            file_type,
            ncmds,
            size_of_cmds,
            flags,
            reserved,
        })
    }
}

impl Debug for MachHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MachHeader")
            .field("magic", &self.magic)
            .field("cpu_type", &self.cpu_type)
            .field("cpu_subtype", &self.cpu_subtype)
            .field("file_type", &self.file_type)
            .field("ncmds", &self.ncmds)
            .field("size_of_cmds", &self.size_of_cmds)
            .field("flags", &self.flags)
            .field("reserved", &self.reserved)
            .finish()
    }
}

impl MachHeader {
    pub fn magic(&self) -> Magic {
        self.magic
    }

    pub fn cpu_type(&self) -> CPUType {
        self.cpu_type
    }

    pub fn cpu_subtype(&self) -> CPUSubtype {
        self.cpu_subtype & !CPU_SUBTYPE_MASK
    }

    pub fn file_type(&self) -> u32 {
        self.file_type
    }

    pub fn ncmds(&self) -> u32 {
        self.ncmds
    }

    pub fn size_of_cmds(&self) -> u32 {
        self.size_of_cmds
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn reserved(&self) -> u32 {
        self.reserved
    }
}
