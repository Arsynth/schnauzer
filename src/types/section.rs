use super::primitives::*;

use super::auto_enum_fields::*;
use super::RcReader;
use super::Result;
use schnauzer_derive::AutoEnumFields;
use scroll::ctx::{FromCtx, SizeWith};
use scroll::IOread;
use std::fmt::Debug;

/// Both `section` and `section_64`
#[derive(Debug, AutoEnumFields)]
pub struct Section {
    pub sectname: Str16Bytes,
    pub segname: Str16Bytes,
    pub addr: Hu64,
    pub size: Hu64,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: Hu32,
    pub reserved1: u32,
    pub reserved2: u32,
    /// Only for `section_64`
    pub reserved3: u32opt,
}

impl Section {
    pub(super) fn parse(reader: RcReader, ctx: X64Context) -> Result<Self> {
        let endian = ctx.endian().clone();
        let mut reader_mut = reader.borrow_mut();

        let sectname: Str16Bytes = reader_mut.ioread_with(endian)?;
        let segname: Str16Bytes = reader_mut.ioread_with(endian)?;
        let addr: Hu64 = reader_mut.ioread_with(ctx)?;
        let size: Hu64 = reader_mut.ioread_with(ctx)?;
        let offset: u32 = reader_mut.ioread_with(endian)?;
        let align: u32 = reader_mut.ioread_with(endian)?;
        let reloff: u32 = reader_mut.ioread_with(endian)?;
        let nreloc: u32 = reader_mut.ioread_with(endian)?;
        let flags: Hu32 = reader_mut.ioread_with(endian)?;
        let reserved1: u32 = reader_mut.ioread_with(endian)?;
        let reserved2: u32 = reader_mut.ioread_with(endian)?;
        let reserved3: u32opt = reader_mut.ioread_with(ctx)?;

        Ok(Self {
            sectname,
            segname,
            addr,
            size,
            offset,
            align,
            reloff,
            nreloc,
            flags,
            reserved1,
            reserved2,
            reserved3,
        })
    }
}
