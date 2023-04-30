use super::RcReader;
use super::Result;
use super::MachHeader;
use super::LoadCommand;

use std::fmt::{Debug};
use std::io::{Seek, SeekFrom};

pub struct MachObject {
    reader: RcReader,

    pub(super) header: MachHeader,
    pub(super) commands_offset: usize,
}

impl MachObject {
    pub(super) fn parse(reader: RcReader, base_offset: usize) -> Result<MachObject> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;
        // We should drop it explicitly before used in `MachHeader`
        std::mem::drop(reader_mut);

        let header = MachHeader::parse(reader.clone())?;

        let mut reader_mut = reader.borrow_mut();
        // After reading the header `reader_mut` should stand on
        // start of load commands list
        let commands_offset = reader_mut.stream_position()? as usize;

        Ok(MachObject {
            reader: reader.clone(),
            header: header,
            commands_offset: commands_offset,
        })
    }
}

impl MachObject {
    pub fn header(&self) -> &MachHeader {
        &self.header
    }

    pub fn load_commands_iterator(&self) -> LoadCommandIterator {
        LoadCommandIterator::new(
            self.reader.clone(),
            self.commands_offset,
            self.header.size_of_cmds,
            self.header.magic.endian(),
        )
    }

    pub fn segments_iterator(&self) -> SegmentIterator {
        SegmentIterator
    }
}

impl Debug for MachObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let commands: Vec<LoadCommand> = self.load_commands_iterator().collect();

        f.debug_struct("MachObject")
            .field("header", &self.header)
            .field("commands_offset", &self.commands_offset)
            .field("self.load_commands_iterator()", &commands)
            .finish()
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

        self.current_offset += lc.cmdsize as usize;

        Some(lc)
    }
}

pub struct SegmentIterator;