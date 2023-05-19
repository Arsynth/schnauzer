use super::RcReader;
use super::Result;
/// <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/nlist.h.auto.html>
use crate::LcStr;

use super::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;
use scroll::IOread;

type NlistStr = LcStr;

#[derive(AutoEnumFields)]
pub enum NlistVariant {
    Nlist32(Nlist32),
    Nlist64(Nlist64),
}

impl NlistVariant {
    pub fn is_64(&self) -> bool {
        match self {
            NlistVariant::Nlist32(_) => false,
            NlistVariant::Nlist64(_) => true,
        }
    }
}

#[repr(C)]
#[derive(AutoEnumFields)]
pub struct Nlist32 {
    /// In the original `nlist` struct this field is uniun - `n_un`
    pub n_strx: u32,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u32,

    /// Depends on `n_strx`, `stroff` of `LcSymtab` [and image offset in file if that in fat file]
    pub name: NlistStr,
}

impl Nlist32 {
    pub(super) fn parse(reader: RcReader, stroff: u64, endian: scroll::Endian) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();

        let n_strx: u32 = reader_mut.ioread_with(endian)?;
        let n_type: u8 = reader_mut.ioread_with(endian)?;
        let n_sect: u8 = reader_mut.ioread_with(endian)?;
        let n_desc: u16 = reader_mut.ioread_with(endian)?;
        let n_value: u32 = reader_mut.ioread_with(endian)?;
        let name = NlistStr {
            reader: reader_clone,
            file_offset: stroff as u32 + n_strx,
        };

        Ok(Self {
            n_strx,
            n_type,
            n_sect,
            n_desc,
            n_value,
            name,
        })
    }
}

#[repr(C)]
#[derive(AutoEnumFields)]
pub struct Nlist64 {
    pub n_strx: u32,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,

    /// Depends on `n_strx`, `stroff` of `LcSymtab` [and image offset in file if that in fat file]
    pub name: NlistStr,
}

impl Nlist64 {
    pub(super) fn parse(reader: RcReader, stroff: u64, endian: scroll::Endian) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();

        let n_strx: u32 = reader_mut.ioread_with(endian)?;
        let n_type: u8 = reader_mut.ioread_with(endian)?;
        let n_sect: u8 = reader_mut.ioread_with(endian)?;
        let n_desc: u16 = reader_mut.ioread_with(endian)?;
        let n_value: u64 = reader_mut.ioread_with(endian)?;
        let name = NlistStr {
            reader: reader_clone,
            file_offset: stroff as u32 + n_strx,
        };

        Ok(Self {
            n_strx,
            n_type,
            n_sect,
            n_desc,
            n_value,
            name,
        })
    }
}
