use super::constants::*;
use scroll::{ctx, Pread};
use std::fmt::{Debug, Display};
use std::mem;

use super::result::Error;
use super::result::Result;

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

impl Magic {
    pub(super) fn raw_data_size() -> usize {
        std::mem::size_of::<u32>()
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

impl<'a> ctx::TryFromCtx<'a, scroll::Endian> for Magic {
    type Error = super::result::Error;

    fn try_from_ctx(
        from: &'a [u8],
        ctx: scroll::Endian,
    ) -> std::result::Result<(Self, usize), Self::Error> {
        if from.len() < std::mem::size_of::<u32>() {
            return Err(Self::Error::BadBufferLength);
        }

        let raw_value: u32 = from.pread_with(0, ctx)?;

        let result: Magic = match raw_value.try_into() {
            Ok(m) => m,
            Err(e) => {
                return Err(Self::Error::Other(Box::new(e)));
            }
        };

        Ok((result, Magic::raw_data_size()))
    }
}

impl Magic {
    fn endian(&self) -> scroll::Endian {
        if self.is_reverse() {
            scroll::LE
        } else {
            scroll::BE
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

#[derive(Debug, PartialEq)]
pub enum ObjectType<'a> {
    Fat(FatHeader<'a>),
    MachHeader(MachHeader<'a>),
}

impl<'a> ObjectType<'a> {
    pub(super) fn parse(buf: &[u8]) -> Result<ObjectType> {
        let magic = buf.pread_with::<Magic>(0, scroll::BE)?;
        if magic.is_fat() {
            let header = FatHeader::parse(buf)?;
            Ok(ObjectType::Fat(header))
        } else {
            let header = MachHeader::parse(buf, 0)?;
            Ok(ObjectType::MachHeader(header))
        }
    }
}

#[derive(PartialEq)]
pub struct FatHeader<'a> {
    pub(super) buf: &'a [u8],
    arch_list_offset: usize,
    pub(super) nfat_arch: u32,
}

impl<'a> FatHeader<'a> {
    fn parse(buf: &[u8]) -> Result<FatHeader> {
        let offset = BYTES_PER_MAGIC;
        let nfat_arch: u32 = buf.pread_with(offset, scroll::BE)?;

        Ok(FatHeader {
            buf: buf,
            arch_list_offset: BYTES_PER_FAT_HEADER,
            nfat_arch,
        })
    }
}

impl<'a> FatHeader<'a> {
    pub fn arch_iterator(&self) -> FatArchIterator<'a> {
        FatArchIterator::build(self.buf, self.nfat_arch, self.arch_list_offset).unwrap()
    }
}

impl<'a> Debug for FatHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let archs: Vec<FatArch> = self.arch_iterator().collect();

        f.debug_struct("FatHeader")
            .field("buf", &self.buf.len())
            .field("arch_list_offset", &self.arch_list_offset)
            .field("nfat_arch", &self.nfat_arch)
            .field("arch_iterator()", &archs)
            .finish()
    }
}

pub struct FatArchIterator<'a> {
    buf: &'a [u8],
    nfat_arch: u32,

    base_offset: usize,

    current: usize,
}

impl<'a> FatArchIterator<'a> {
    fn build(buf: &'a [u8], nfat_arch: u32, base_offset: usize) -> Result<FatArchIterator<'a>> {
        if buf.len() < nfat_arch as usize * BYTES_PER_FAT_ARCH {
            return Err(Error::BadBufferLength);
        }

        Ok(FatArchIterator {
            buf: buf,
            nfat_arch: nfat_arch,
            base_offset: base_offset,
            current: 0,
        })
    }
}

impl<'a> Iterator for FatArchIterator<'a> {
    type Item = FatArch<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.nfat_arch as usize {
            return None;
        }
        let offset = self.base_offset + BYTES_PER_FAT_ARCH * self.current;

        self.current += 1;

        Some(FatArch::parse(self.buf, offset).unwrap())
    }
}

#[derive(PartialEq)]
pub struct FatArch<'a> {
    pub(super) buf: &'a [u8],

    pub(super) cpu_type: CPUType,
    pub(super) cpu_subtype: CPUSubtype,
    pub(super) offset: u32,
    pub(super) size: u32,
    pub(super) align: u32,
}

impl<'a> FatArch<'a> {
    fn parse(buf: &'a [u8], base_offset: usize) -> Result<FatArch<'a>> {
        let mut c_offset: usize = base_offset;

        if buf.len() < BYTES_PER_FAT_ARCH {
            return Err(Error::BadBufferLength);
        }

        let cpu_type: CPUType = buf.pread_with(c_offset, scroll::BE)?;
        c_offset += mem::size_of::<CPUType>();

        let cpu_subtype: CPUSubtype = buf.pread_with(c_offset, scroll::BE)?;
        c_offset += mem::size_of::<CPUSubtype>();

        let offset: u32 = buf.pread_with(c_offset, scroll::BE)?;
        c_offset += mem::size_of::<u32>();

        let size: u32 = buf.pread_with(c_offset, scroll::BE)?;
        c_offset += mem::size_of::<u32>();

        let align: u32 = buf.pread_with(c_offset, scroll::BE)?;

        Ok(FatArch {
            buf,
            cpu_type: cpu_type,
            cpu_subtype: cpu_subtype,
            offset: offset,
            size: size,
            align: align,
        })
    }
}

impl<'a> FatArch<'a> {
    pub fn mach_header(&self) -> Result<MachHeader<'a>> {
        MachHeader::parse(self.buf, self.offset as usize)
    }
}

impl<'a> Debug for FatArch<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("FatArch");

        s.field("buf", &self.buf.len())
            .field("cpu_type", &self.cpu_type)
            .field("cpu_subtype", &self.cpu_subtype)
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("align", &self.align);

        if let Result::Ok(h) = MachHeader::parse(self.buf, self.offset as usize) {
            s.field("mach_header()", &h);
        }

        s.finish()
    }
}

impl<'a> FatArch<'a> {
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

#[derive(PartialEq)]
pub struct MachHeader<'a> {
    buf: &'a [u8],
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

impl<'a> MachHeader<'a> {
    fn parse(buf: &'a [u8], base_offset: usize) -> Result<MachHeader<'a>> {
        let mut offset = base_offset;

        let mut ctx = scroll::BE;

        let magic: Magic = buf.pread_with(offset, ctx)?;
        offset += BYTES_PER_MAGIC;

        if magic.is_reverse() {
            ctx = scroll::LE;
        }
        let ctx = ctx;

        let cpu_type: CPUType = buf.pread_with(offset, ctx)?;
        offset += mem::size_of::<CPUType>();

        let cpu_subtype: CPUSubtype = buf.pread_with(offset, ctx)?;
        offset += mem::size_of::<CPUSubtype>();

        let file_type: u32 = buf.pread_with(offset, ctx)?;
        offset += mem::size_of::<u32>();

        let ncmds: u32 = buf.pread_with(offset, ctx)?;
        offset += mem::size_of::<u32>();

        let size_of_cmds: u32 = buf.pread_with(offset, ctx)?;
        offset += mem::size_of::<u32>();

        let flags: u32 = buf.pread_with(offset, ctx)?;
        offset += mem::size_of::<u32>();

        let mut reserved = 0u32;
        if magic.is_64() {
            reserved = buf.pread_with(offset, ctx)?;
        }
        offset += mem::size_of::<u32>();

        Ok(MachHeader {
            buf: buf,
            commands_offset: offset as usize,
            magic: magic,
            cpu_type: cpu_type,
            cpu_subtype: cpu_subtype,
            file_type: file_type,
            ncmds: ncmds,
            size_of_cmds: size_of_cmds,
            flags: flags,
            reserved: reserved,
        })
    }
}

impl<'a> Debug for MachHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let commands: Vec<LoadCommand> = self.load_commands_iterator().collect();

        f.debug_struct("MachHeader")
            .field("buf", &self.buf.len())
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

impl<'a> MachHeader<'a> {
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

impl<'a> MachHeader<'a> {
    pub fn load_commands_iterator(&self) -> LoadCommandIterator {
        LoadCommandIterator::new(
            self.buf,
            self.commands_offset,
            self.size_of_cmds,
            self.magic.endian(),
        )
    }
}

pub struct LoadCommandIterator<'a> {
    buf: &'a [u8],
    current_offset: usize,
    end_offset: usize,
    endian: scroll::Endian,
}

impl<'a> LoadCommandIterator<'a> {
    fn new(
        buf: &'a [u8],
        base_offset: usize,
        size_of_cmds: u32,
        endian: scroll::Endian,
    ) -> LoadCommandIterator {
        LoadCommandIterator {
            buf: buf,
            current_offset: base_offset,
            end_offset: base_offset + size_of_cmds as usize,
            endian: endian,
        }
    }
}

impl<'a> Iterator for LoadCommandIterator<'a> {
    type Item = LoadCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_offset >= self.end_offset as usize {
            return None;
        }

        let lc = LoadCommand::parse(self.buf, self.current_offset, self.endian).unwrap();

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
    fn parse(buf: &[u8], base_offset: usize, endian: scroll::Endian) -> Result<LoadCommand> {
        let mut offset = base_offset;

        let cmd: u32 = buf.pread_with(offset, endian)?;
        offset += mem::size_of::<u32>();

        let cmd_size: u32 = buf.pread_with(offset, endian)?;

        Ok(LoadCommand {
            cmd: cmd,
            cmd_size: cmd_size,
        })
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
