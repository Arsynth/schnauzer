use super::RcReader;
use scroll::IOread;
use super::constants::*;
use super::Result;
use super::MachObject;

use std::fmt::{Debug};
use std::io::{Seek, SeekFrom};

use super::utils;

pub struct FatArch {
    pub reader: RcReader,

    pub cputype: CPUType,
    pub cpusubtype: CPUSubtype,
    pub offset: u32,
    pub size: u32,
    pub align: u32,
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
            cputype: cpu_type,
            cpusubtype: cpu_subtype,
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

impl FatArch {
    pub fn masked_cpu_subtype(&self) -> CPUSubtype {
        utils::masked_cpu_subtype(self.cpusubtype)
         & !CPU_SUBTYPE_MASK
    }

    pub fn feature_flags(&self) -> u32 {
        utils::feature_flags(self.cpusubtype)
    }

    pub fn is_64(&self) -> bool {
        utils::is_64(self.cputype)
    }
}

impl Debug for FatArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FatArch");

        s.field("cpu_type", &self.cputype)
            .field("cpu_subtype", &self.cpusubtype)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("align", &self.align);

        if let Result::Ok(h) = MachObject::parse(self.reader.clone(), self.offset as usize) {
            s.field("mach_header()", &h);
        }

        s.finish()
    }
}
