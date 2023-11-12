use crate::RcReader;
use crate::Result;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

/// `thread_command`
#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcThread {
    pub flavor: u32,
    pub count: u32,
    /* struct XXX_thread_state state   thread state for this flavor */
    /* ... */

    state_offset: u64
}

impl LcThread {
    pub(super) fn parse(reader: RcReader, base_offset: usize, endian: scroll::Endian) -> Result<Self> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let flavor: u32 = reader_mut.ioread_with(endian)?;
        let count: u32 = reader_mut.ioread_with(endian)?;

        let state_offset = reader_mut.stream_position()?;
        
        Ok(LcThread { flavor, count, state_offset })
    }

    pub fn get_state_offset(&self) -> u64 {
        self.state_offset
    }
}

impl Debug for LcThread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcThread")
            .field("flavor", &self.flavor)
            .field("count", &self.count)
            .finish()
    }
}