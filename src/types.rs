use super::constants::*;
use scroll::IOread;
use std::fmt::{Debug, Display};
use std::io::{Seek, SeekFrom};

use super::result::Result;

use super::reader::RcReader;

#[derive(PartialEq)]
pub enum Magic {
    Fat,
    FatReverse,
    Bin32,
    Bin32Reverse,
    Bin64,
    Bin64Reverse,
}

impl Magic {
    pub(super) fn raw_value(&self) -> u32 {
        match self {
            Magic::Fat => 0xcafebabe,
            Magic::FatReverse => 0xbebafeca,
            Magic::Bin32 => 0xfeedface,
            Magic::Bin32Reverse => 0xcefaedfe,
            Magic::Bin64 => 0xfeedfacf,
            Magic::Bin64Reverse => 0xcffaedfe,
        }
    }

    pub fn is_fat(&self) -> bool {
        match self {
            Self::Fat | Self::FatReverse => true,
            _ => false,
        }
    }

    pub fn is_reverse(&self) -> bool {
        match self {
            Magic::FatReverse => true,
            Magic::Bin32Reverse => true,
            Magic::Bin64Reverse => true,
            _ => false,
        }
    }

    pub fn is_64(&self) -> bool {
        match self {
            Self::Bin64 | Self::Bin64Reverse => true,
            _ => false,
        }
    }
}

impl TryInto<Magic> for u32 {
    type Error = super::result::Error;

    fn try_into(self) -> std::result::Result<Magic, Self::Error> {
        match self {
            0xcafebabe => Ok(Magic::Fat),
            0xbebafeca => Ok(Magic::FatReverse),
            0xfeedface => Ok(Magic::Bin32),
            0xcefaedfe => Ok(Magic::Bin32Reverse),
            0xfeedfacf => Ok(Magic::Bin64),
            0xcffaedfe => Ok(Magic::Bin64Reverse),
            _ => Err(Self::Error::BadMagic(self)),
        }
    }
}

impl Magic {
    fn endian(&self) -> scroll::Endian {
        if self.is_fat() || !self.is_reverse() {
            scroll::BE
        } else {
            scroll::LE
        }
    }
}

impl Clone for Magic {
    fn clone(&self) -> Self {
        match self {
            Self::Fat => Self::Fat,
            Self::FatReverse => Self::FatReverse,
            Self::Bin32 => Self::Bin32,
            Self::Bin32Reverse => Self::Bin32Reverse,
            Self::Bin64 => Self::Bin64,
            Self::Bin64Reverse => Self::Bin64Reverse,
        }
    }
}

impl Copy for Magic {}

impl Display for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.raw_value())
    }
}

impl Debug for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.raw_value())
    }
}

#[derive(Debug)]
pub enum ObjectType {
    Fat(FatHeader),
    MachHeader(MachHeader),
}

impl ObjectType {
    pub(super) fn parse(reader: RcReader) -> Result<ObjectType> {
        let magic = reader.borrow_mut().ioread_with::<u32>(scroll::BE)?;
        let magic: Magic = magic.try_into()?;
        if magic.is_fat() {
            let header = FatHeader::parse(reader.clone())?;
            Ok(ObjectType::Fat(header))
        } else {
            let header = MachHeader::parse(reader.clone(), 0)?;
            Ok(ObjectType::MachHeader(header))
        }
    }
}

pub struct FatHeader {
    pub(super) reader: RcReader,
    arch_list_offset: usize,
    pub(super) nfat_arch: u32,
}

impl FatHeader {
    fn parse(reader: RcReader) -> Result<FatHeader> {
        let offset = BYTES_PER_MAGIC;
        reader.borrow_mut().seek(SeekFrom::Start(offset as u64))?;
        let nfat_arch: u32 = reader.borrow_mut().ioread_with(scroll::BE)?;

        Ok(FatHeader {
            reader: reader.clone(),
            arch_list_offset: BYTES_PER_FAT_HEADER,
            nfat_arch,
        })
    }
}

impl FatHeader {
    pub fn arch_iterator(&self) -> FatArchIterator {
        FatArchIterator::build(self.reader.clone(), self.nfat_arch, self.arch_list_offset).unwrap()
    }
}

impl Debug for FatHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let archs: Vec<FatArch> = self.arch_iterator().collect();

        f.debug_struct("FatHeader")
            .field("arch_list_offset", &self.arch_list_offset)
            .field("nfat_arch", &self.nfat_arch)
            .field("arch_iterator()", &archs)
            .finish()
    }
}

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

pub struct FatArch {
    pub(super) reader: RcReader,

    pub(super) cpu_type: CPUType,
    pub(super) cpu_subtype: CPUSubtype,
    pub(super) offset: u32,
    pub(super) size: u32,
    pub(super) align: u32,
}

impl FatArch {
    fn parse(reader: RcReader, base_offset: usize) -> Result<FatArch> {
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
            cpu_type,
            cpu_subtype,
            offset,
            size,
            align,
        })
    }
}

impl FatArch {
    pub fn mach_header(&self) -> Result<MachHeader> {
        MachHeader::parse(self.reader.clone(), self.offset as usize)
    }
}

impl Debug for FatArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FatArch");

        s.field("cpu_type", &self.cpu_type)
            .field("cpu_subtype", &self.cpu_subtype)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("align", &self.align);

        if let Result::Ok(h) = MachHeader::parse(self.reader.clone(), self.offset as usize) {
            s.field("mach_header()", &h);
        }

        s.finish()
    }
}

impl FatArch {
    pub fn cpu_type(&self) -> CPUType {
        self.cpu_type
    }

    pub fn cpu_subtype(&self) -> CPUSubtype {
        self.cpu_subtype & !CPU_SUBTYPE_MASK
    }

    pub fn feature_flags(&self) -> u32 {
        (self.cpu_subtype & CPU_SUBTYPE_MASK) >> 24
    }

    pub fn is_64(&self) -> bool {
        (self.cpu_type & CPU_ARCH_ABI64) == CPU_ARCH_ABI64
    }
}

pub struct MachHeader {
    reader: RcReader,
    commands_offset: usize,

    pub(super) magic: Magic,
    pub(super) cpu_type: CPUType,
    pub(super) cpu_subtype: CPUSubtype,
    pub(super) file_type: u32,
    pub(super) ncmds: u32,
    pub(super) size_of_cmds: u32,
    pub(super) flags: u32,
    pub(super) reserved: u32, // For 64 bit headers
}

impl MachHeader {
    fn parse(reader: RcReader, base_offset: usize) -> Result<MachHeader> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let mut ctx = scroll::BE;

        let magic: u32 = reader_mut.ioread_with(ctx)?;
        let magic: Magic = magic.try_into()?;

        if magic.is_reverse() {
            ctx = scroll::LE;
        }
        let ctx = ctx;

        let cpu_type: CPUType = reader_mut.ioread_with(ctx)?;
        let cpu_subtype: CPUSubtype = reader_mut.ioread_with(ctx)?;
        let file_type: u32 = reader_mut.ioread_with(ctx)?;
        let ncmds: u32 = reader_mut.ioread_with(ctx)?;
        let size_of_cmds: u32 = reader_mut.ioread_with(ctx)?;
        let flags: u32 = reader_mut.ioread_with(ctx)?;

        let mut reserved = 0u32;
        if magic.is_64() {
            reserved = reader_mut.ioread_with(ctx)?;
        }

        let commands_offset = reader_mut.stream_position()? as usize;

        Ok(MachHeader {
            reader: reader.clone(),
            commands_offset,
            magic,
            cpu_type,
            cpu_subtype,
            file_type,
            ncmds,
            size_of_cmds,
            flags,
            reserved,
        })
    }
}

impl Debug for MachHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let commands: Vec<LoadCommand> = self.load_commands_iterator().collect();

        f.debug_struct("MachHeader")
            .field("magic", &self.magic)
            .field("cpu_type", &self.cpu_type)
            .field("cpu_subtype", &self.cpu_subtype)
            .field("file_type", &self.file_type)
            .field("ncmds", &self.ncmds)
            .field("size_of_cmds", &self.size_of_cmds)
            .field("flags", &self.flags)
            .field("reserved", &self.reserved)
            .field("self.load_commands_iterator()", &commands)
            .finish()
    }
}

impl MachHeader {
    pub fn magic(&self) -> Magic {
        self.magic
    }

    pub fn cpu_type(&self) -> CPUType {
        self.cpu_type
    }

    pub fn cpu_subtype(&self) -> CPUSubtype {
        self.cpu_subtype & !CPU_SUBTYPE_MASK
    }

    pub fn file_type(&self) -> u32 {
        self.file_type
    }

    pub fn ncmds(&self) -> u32 {
        self.ncmds
    }

    pub fn size_of_cmds(&self) -> u32 {
        self.size_of_cmds
    }

    pub fn flags(&self) -> u32 {
        self.flags
    }

    pub fn reserved(&self) -> u32 {
        self.reserved
    }
}

impl MachHeader {
    pub fn load_commands_iterator(&self) -> LoadCommandIterator {
        LoadCommandIterator::new(
            self.reader.clone(),
            self.commands_offset,
            self.size_of_cmds,
            self.magic.endian(),
        )
    }
}

pub struct LoadCommandIterator {
    reader: RcReader,
    current_offset: usize,
    end_offset: usize,
    endian: scroll::Endian,
}

impl LoadCommandIterator {
    fn new(
        reader: RcReader,
        base_offset: usize,
        size_of_cmds: u32,
        endian: scroll::Endian,
    ) -> LoadCommandIterator {
        LoadCommandIterator {
            reader: reader,
            current_offset: base_offset,
            end_offset: base_offset + size_of_cmds as usize,
            endian: endian,
        }
    }
}

impl Iterator for LoadCommandIterator {
    type Item = LoadCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset >= self.end_offset as usize {
            return None;
        }

        let lc = LoadCommand::parse(self.reader.clone(), self.current_offset, self.endian).unwrap();

        self.current_offset += lc.cmd_size as usize;

        Some(lc)
    }
}

#[derive(Debug)]
pub struct LoadCommand {
    cmd: u32,
    cmd_size: u32,
}

impl LoadCommand {
    fn parse(
        reader: RcReader,
        base_offset: usize,
        endian: scroll::Endian,
    ) -> Result<LoadCommand> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let cmd: u32 = reader_mut.ioread_with(endian)?;
        let cmd_size: u32 = reader_mut.ioread_with(endian)?;

        Ok(LoadCommand { cmd, cmd_size })
    }
}

impl LoadCommand {
    pub fn cmd(&self) -> LoadCommandType {
        self.cmd
    }

    pub fn cmd_size(&self) -> u32 {
        self.cmd_size
    }
}

pub enum SegmentType {
    Segment32(Segment32),
    Segment64(Segment64),
}
pub struct Segment32;
pub struct Segment64;

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_magic_consistence() {
        check_magic_interchangeability(Magic::Fat);
        check_magic_interchangeability(Magic::FatReverse);
        check_magic_interchangeability(Magic::Bin32);
        check_magic_interchangeability(Magic::Bin32Reverse);
        check_magic_interchangeability(Magic::Bin64);
        check_magic_interchangeability(Magic::Bin64Reverse);
    }

    fn check_magic_interchangeability(magic: Magic) {
        let raw_magic = magic.raw_value();
        let from_raw: Magic = raw_magic.try_into().unwrap_or_else(|_| {
            panic!(
                "Magic '{:#09x}' can not be converted to concrete type",
                raw_magic
            );
        });

        assert_eq!(
            raw_magic,
            from_raw.raw_value(),
            "Expected '{:#09x}', got '{:#09x}'",
            raw_magic,
            from_raw.raw_value()
        );
    }
}
