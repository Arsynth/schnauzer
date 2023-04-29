use super::RcReader;
use scroll::IOread;
use super::constants::*;
use super::Result;
use super::MachObject;

use std::fmt::{Debug};
use std::io::{Seek, SeekFrom};

pub struct FatArch {
    pub(crate) reader: RcReader,

    pub(crate) cpu_type: CPUType,
    pub(crate) cpu_subtype: CPUSubtype,
    pub(crate) offset: u32,
    pub(crate) size: u32,
    pub(crate) align: u32,
}

impl FatArch {
    pub(super) fn parse(reader: RcReader, base_offset: usize) -> Result<FatArch> {
        const ENDIAN: scroll::Endian = scroll::BE;
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let cpu_type: CPUType = reader_mut.ioread_with(ENDIAN)?;
        let cpu_subtype: CPUSubtype = reader_mut.ioread_with(ENDIAN)?;
        let offset: u32 = reader_mut.ioread_with(ENDIAN)?;
        let size: u32 = reader_mut.ioread_with(ENDIAN)?;
        let align: u32 = reader_mut.ioread_with(ENDIAN)?;

        Ok(FatArch {
            reader: reader.clone(),
            cpu_type,
            cpu_subtype,
            offset,
            size,
            align,
        })
    }
}

impl FatArch {
    pub fn object(&self) -> Result<MachObject> {
        MachObject::parse(self.reader.clone(), self.offset as usize)
    }
}

impl Debug for FatArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FatArch");

        s.field("cpu_type", &self.cpu_type)
            .field("cpu_subtype", &self.cpu_subtype)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("align", &self.align);

        if let Result::Ok(h) = MachObject::parse(self.reader.clone(), self.offset as usize) {
            s.field("mach_header()", &h);
        }

        s.finish()
    }
}

impl FatArch {
    pub fn cpu_type(&self) -> CPUType {
        self.cpu_type
    }

    pub fn cpu_subtype(&self) -> CPUSubtype {
        self.cpu_subtype & !CPU_SUBTYPE_MASK
    }

    pub fn feature_flags(&self) -> u32 {
        (self.cpu_subtype & CPU_SUBTYPE_MASK) >> 24
    }

    pub fn is_64(&self) -> bool {
        (self.cpu_type & CPU_ARCH_ABI64) == CPU_ARCH_ABI64
    }
}