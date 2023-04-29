use super::RcReader;
use scroll::IOread;
use super::constants::*;
use super::Result;

use std::fmt::{Debug};
use std::io::{Seek, SeekFrom};

#[derive(Debug)]
pub struct LoadCommand {
    pub(super) cmd: u32,
    pub(super) cmd_size: u32,
}

impl LoadCommand {
    pub(super) fn parse(reader: RcReader, base_offset: usize, endian: scroll::Endian) -> Result<LoadCommand> {
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