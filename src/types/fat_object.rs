use crate::MachObject;

use super::constants::*;
use super::FatArch;
use super::RcReader;
use super::Result;
use scroll::IOread;

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

/// Represents `fat_header` (but does not include magic)
/// Some files contains multiple architectures
pub struct FatObject {
    pub(super) reader: RcReader,

    pub nfat_arch: u32,
    arch_list_offset: usize,
}

impl FatObject {
    pub(super) fn parse(reader: RcReader) -> Result<FatObject> {
        let offset = BYTES_PER_MAGIC;
        reader.borrow_mut().seek(SeekFrom::Start(offset as u64))?;
        let nfat_arch: u32 = reader.borrow_mut().ioread_with(scroll::BE)?;

        Ok(FatObject {
            reader: reader.clone(),
            arch_list_offset: BYTES_PER_FAT_HEADER,
            nfat_arch,
        })
    }
}

impl FatObject {
    /// Iterate over architectures
    pub fn arch_iterator(&self) -> FatArchIterator {
        FatArchIterator::build(self.reader.clone(), self.nfat_arch, self.arch_list_offset).unwrap()
    }

    /// Collect all existing objects
    pub fn objects(&self) -> Vec<MachObject> {
        self.arch_iterator().filter_map(|a| match a.object() {
            Ok(o) => Some(o),
            Err(_) => None,
        }).collect()
    }
}

impl Debug for FatObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let archs: Vec<FatArch> = self.arch_iterator().collect();

        f.debug_struct("FatHeader")
            .field("arch_list_offset", &self.arch_list_offset)
            .field("nfat_arch", &self.nfat_arch)
            .field("arch_iterator()", &archs)
            .finish()
    }
}

/// Iterator over fat architectures
pub struct FatArchIterator {
    reader: RcReader,
    nfat_arch: u32,

    base_offset: usize,

    current: usize,
}

impl FatArchIterator {
    fn build(reader: RcReader, nfat_arch: u32, base_offset: usize) -> Result<FatArchIterator> {
        Ok(FatArchIterator {
            reader,
            nfat_arch,
            base_offset,
            current: 0,
        })
    }
}

impl Iterator for FatArchIterator {
    type Item = FatArch;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.nfat_arch as usize {
            return None;
        }
        let offset = self.base_offset + BYTES_PER_FAT_ARCH * self.current;

        self.current += 1;

        Some(FatArch::parse(self.reader.clone(), offset).unwrap())
    }
}
