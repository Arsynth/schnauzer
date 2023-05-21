use crate::RcReader;
use crate::Result;
/// <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/nlist.h.auto.html>
use crate::LcStr;
use super::U64U32;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;
use scroll::IOread;

type NlistStr = LcStr;
type Nvalue = U64U32;

#[repr(C)]
#[derive(AutoEnumFields)]
pub struct Nlist {
    /// In the original `nlist` struct this field is uniun - `n_un`
    pub n_strx: u32,
    pub n_type: u8,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: Nvalue,

    /// Depends on `n_strx`, `stroff` of `LcSymtab` [and image offset in file if that in fat file]
    pub name: NlistStr,
}

impl Nlist {
    pub(super) fn parse(reader: RcReader, stroff: u64, is_64: bool, endian: scroll::Endian) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();

        let n_strx: u32 = reader_mut.ioread_with(endian)?;
        let n_type: u8 = reader_mut.ioread_with(endian)?;
        let n_sect: u8 = reader_mut.ioread_with(endian)?;
        let n_desc: u16 = reader_mut.ioread_with(endian)?;
        let n_value: Nvalue = if is_64 {
            let val: u64 = reader_mut.ioread_with(endian)?;
            Nvalue::U64(val)
        } else {
            let val: u32 = reader_mut.ioread_with(endian)?;
            Nvalue::U32(val)
        };
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
