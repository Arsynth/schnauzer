use crate::RcReader;
use crate::Result;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::LcStr;
use super::BitVec;

/// `prebound_dylib_command`
#[repr(C)]
#[derive(Debug, AutoEnumFields)]
pub struct LcPreboundDylib {
    pub name: LcStr,
    pub nmodules: u32,
    pub linked_modules: BitVec,
}

impl LcPreboundDylib {
    pub(super) fn parse(
        reader: RcReader,
        command_offset: usize,
        base_offset: usize,
        endian: scroll::Endian,
    ) -> Result<Self> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let name_offset: u32 = reader_mut.ioread_with(endian)?;
        let nmodules: u32 = reader_mut.ioread_with(endian)?;
        let linked_modules_offset: u32 = reader_mut.ioread_with(endian)?;

        let name_offset = name_offset + command_offset as u32;
        let linked_modules_offset = linked_modules_offset + command_offset as u32;

        std::mem::drop(reader_mut);

        let name = LcStr {
            reader: reader.clone(),
            file_offset: name_offset,
        };

        let linked_modules = BitVec {
            reader: reader.clone(),
            file_offset: linked_modules_offset,
            bytecount: nmodules,
        };

        Ok(LcPreboundDylib {
            name,
            nmodules,
            linked_modules,
        })
    }
}