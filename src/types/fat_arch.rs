use super::RcReader;
use scroll::IOread;
use super::primitives::*;
use super::Result;
use super::MachObject;

use std::fmt::{Debug};
use std::io::{Seek, SeekFrom};

use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

#[derive(AutoEnumFields)]
pub struct FatArch {
    pub(crate) reader: RcReader,

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

    pub fn machine(&self) -> Machine {
        Machine::new(self.cputype, self.cpusubtype)
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
