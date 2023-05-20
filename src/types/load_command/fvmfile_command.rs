use crate::RcReader;
use crate::Result;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::LcStr;

/// `fvmfile_command`
#[repr(C)]
#[derive(Debug, AutoEnumFields)]
pub struct LcFvmFile {
    pub name: LcStr,
    pub header_addr: u32,
}

impl LcFvmFile {
    pub(super) fn parse(
        reader: RcReader,
        command_offset: usize,
        base_offset: usize,
        endian: scroll::Endian,
    ) -> Result<Self> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let name_offset: u32 = reader_mut.ioread_with(endian)?;
        let header_addr: u32 = reader_mut.ioread_with(endian)?;

        let name_offset = name_offset + command_offset as u32;
        std::mem::drop(reader_mut);

        let name = LcStr {
            reader: reader.clone(),
            file_offset: name_offset,
        };

        Ok(LcFvmFile { name, header_addr })
    }
}