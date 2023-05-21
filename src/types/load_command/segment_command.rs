use crate::RcReader;
use crate::Result;
use crate::constants::*;
use crate::primitives::*;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

use super::Section32;
use super::Section64;

/// `segment_command`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcSegment32 {
    reader: RcReader,

    pub segname: Segname,
    pub vmaddr: Hu32,
    pub vmsize: Hu32,
    pub fileoff: u32,
    pub filesize: u32,
    pub maxprot: VmProt,
    pub initprot: VmProt,
    pub nsects: u32,
    pub flags: Hu32,

    sects_offset: u64,
    endian: scroll::Endian,
}

impl LcSegment32 {
    pub(super) fn parse(reader: RcReader, base_offset: usize, endian: scroll::Endian) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let segname: Segname = reader_mut.ioread_with(endian)?;
        let vmaddr: Hu32 = reader_mut.ioread_with(endian)?;
        let vmsize: Hu32 = reader_mut.ioread_with(endian)?;
        let fileoff: u32 = reader_mut.ioread_with(endian)?;
        let filesize: u32 = reader_mut.ioread_with(endian)?;
        let maxprot: VmProt = reader_mut.ioread_with(endian)?;
        let initprot: VmProt = reader_mut.ioread_with(endian)?;
        let nsects: u32 = reader_mut.ioread_with(endian)?;
        let flags: Hu32 = reader_mut.ioread_with(endian)?;

        let sects_offset = reader_mut.stream_position()?;

        Ok(LcSegment32 {
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
            endian,
        })
    }
}

impl Debug for LcSegment32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcSegment")
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

impl LcSegment32 {
    pub fn sections_iterator(&self) -> Section32Iterator {
        Section32Iterator::new(
            self.reader.clone(),
            self.nsects,
            self.sects_offset,
            self.endian,
        )
    }
}

pub struct Section32Iterator {
    reader: RcReader,

    nsects: u32,
    base_offset: u64,
    endian: scroll::Endian,

    current: u32,
}

impl Section32Iterator {
    fn new(reader: RcReader, nsects: u32, base_offset: u64, endian: scroll::Endian) -> Self {
        Section32Iterator {
            reader,
            nsects,
            base_offset,
            current: 0,
            endian,
        }
    }
}

impl Iterator for Section32Iterator {
    type Item = Section32;

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

        match reader_mut.ioread_with::<Section32>(self.endian) {
            Ok(sect) => Some(sect),
            Err(_) => return None,
        }
    }
}

/// `segment_command_64`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcSegment64 {
    reader: RcReader,

    pub segname: Segname,
    pub vmaddr: Hu64,
    pub vmsize: Hu64,
    pub fileoff: u64,
    pub filesize: u64,
    pub maxprot: VmProt,
    pub initprot: VmProt,
    pub nsects: u32,
    pub flags: Hu32,

    sects_offset: u64,
    endian: scroll::Endian,
}

impl LcSegment64 {
    pub(super) fn parse(reader: RcReader, base_offset: usize, endian: scroll::Endian) -> Result<Self> {
        let reader_clone = reader.clone();
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let segname: Segname = reader_mut.ioread_with(endian)?;
        let vmaddr: Hu64 = reader_mut.ioread_with(endian)?;
        let vmsize: Hu64 = reader_mut.ioread_with(endian)?;
        let fileoff: u64 = reader_mut.ioread_with(endian)?;
        let filesize: u64 = reader_mut.ioread_with(endian)?;
        let maxprot: VmProt = reader_mut.ioread_with(endian)?;
        let initprot: VmProt = reader_mut.ioread_with(endian)?;
        let nsects: u32 = reader_mut.ioread_with(endian)?;
        let flags: Hu32 = reader_mut.ioread_with(endian)?;

        let sects_offset = reader_mut.stream_position()?;

        Ok(LcSegment64 {
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
            endian,
        })
    }
}

impl Debug for LcSegment64 {
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

impl LcSegment64 {
    pub fn sections_iterator(&self) -> Section64Iterator {
        Section64Iterator::new(
            self.reader.clone(),
            self.nsects,
            self.sects_offset,
            self.endian,
        )
    }
}

pub struct Section64Iterator {
    reader: RcReader,

    nsects: u32,
    base_offset: u64,
    endian: scroll::Endian,

    current: u32,
}

impl Section64Iterator {
    fn new(reader: RcReader, nsects: u32, base_offset: u64, endian: scroll::Endian) -> Self {
        Section64Iterator {
            reader,
            nsects,
            base_offset,
            current: 0,
            endian,
        }
    }
}

impl Iterator for Section64Iterator {
    type Item = Section64;

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

        match reader_mut.ioread_with::<Section64>(self.endian) {
            Ok(sect) => Some(sect),
            Err(_) => return None,
        }
    }
}