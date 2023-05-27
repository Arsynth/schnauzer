use crate::RcReader;
use crate::Result;
use crate::constants::*;
use crate::primitives::*;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::Section;

/// Both `segment_command` and `segment_command_64`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcSegment {
    reader: RcReader,

    pub segname: Segname,
    pub vmaddr: Hu64,
    pub vmsize: Hu64,
    pub fileoff: u64_io,
    pub filesize: u64_io,
    pub maxprot: VmProt,
    pub initprot: VmProt,
    pub nsects: u32,
    pub flags: Hu32,

    sects_offset: u64,
    ctx: U64Context,
}

impl LcSegment {
    pub(super) fn parse(reader: RcReader, base_offset: usize, ctx: U64Context) -> Result<Self> {
        let endian = *ctx.endian();
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let segname: Segname = reader_mut.ioread_with(endian)?;

        let vmaddr: u64_io = reader_mut.ioread_with(ctx)?;
        let vmaddr = Hu64(vmaddr.0);

        let vmsize: u64_io = reader_mut.ioread_with(ctx)?;
        let vmsize = Hu64(vmsize.0);

        let fileoff: u64_io = reader_mut.ioread_with(ctx)?;
        let filesize: u64_io = reader_mut.ioread_with(ctx)?;
        let maxprot: VmProt = reader_mut.ioread_with(endian)?;
        let initprot: VmProt = reader_mut.ioread_with(endian)?;
        let nsects: u32 = reader_mut.ioread_with(endian)?;
        let flags: Hu32 = reader_mut.ioread_with(endian)?;

        let sects_offset = reader_mut.stream_position()?;

        Ok(LcSegment {
            reader: reader_clone,
            segname,
            vmaddr,
            vmsize,
            fileoff,
            filesize,
            maxprot,
            initprot,
            nsects,
            flags,
            sects_offset,
            ctx,
        })
    }
}

impl Debug for LcSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcSegment64")
            .field("segname", &self.segname)
            .field("vmaddr", &self.vmaddr)
            .field("vmsize", &self.vmsize)
            .field("fileoff", &self.fileoff)
            .field("filesize", &self.filesize)
            .field("maxprot", &self.maxprot)
            .field("initprot", &self.initprot)
            .field("nsects", &self.nsects)
            .field("flags", &self.flags)
            .finish()
    }
}

impl LcSegment {
    pub fn sections_iterator(&self) -> SectionIterator {
        SectionIterator::new(
            self.reader.clone(),
            self.nsects,
            self.sects_offset,
            self.ctx,
        )
    }
}

pub struct SectionIterator {
    reader: RcReader,

    nsects: u32,
    base_offset: u64,
    ctx: U64Context,

    current: u32,
}

impl SectionIterator {
    fn new(reader: RcReader, nsects: u32, base_offset: u64, ctx: U64Context) -> Self {
        SectionIterator {
            reader,
            nsects,
            base_offset,
            current: 0,
            ctx,
        }
    }
}

impl Iterator for SectionIterator {
    type Item = Section;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.nsects {
            return None;
        }

        let offset = self.base_offset + BYTES_PER_SECTION64 as u64 * self.current as u64;
        self.current += 1;

        let mut reader_mut = self.reader.borrow_mut();
        if let Err(_) = reader_mut.seek(SeekFrom::Start(offset as u64)) {
            return None;
        }

        std::mem::drop(reader_mut);

        match Section::parse(self.reader.clone(), self.ctx) {
            Ok(sect) => Some(sect),
            Err(_) => return None,
        }
    }
}