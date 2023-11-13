use crate::RcReader;
use crate::Result;

use scroll::{IOread};

use std::fmt::Debug;
use std::io::{Seek, SeekFrom};
use std::mem::size_of;

use crate::auto_enum_fields::*;
use schnauzer_derive::AutoEnumFields;

const LC_THREAD_FLAVOR_HEADER_SIZE: usize = size_of::<u32>() + size_of::<u32>();

/// `thread_command`
#[repr(C)]
#[derive(AutoEnumFields,Debug)]
pub struct LcThread {
    pub flavors: Vec<LcThreadFlavor>
}

#[repr(C)]
#[derive(AutoEnumFields)]
pub struct LcThreadFlavor {
    pub flavor: u32,
    pub count: u32,
    /* struct XXX_thread_state state   thread state for this flavor */
    /* ... */

    state_offset: u64
}

impl LcThread {
    pub(super) fn parse(reader: RcReader, cmdsize: u32, base_offset: usize, endian: scroll::Endian) -> Result<Self> {
        let mut flavors = Vec::new();
        
        let mut flavor_offset = base_offset;
        loop { 
            let flavor = LcThreadFlavor::parse(&reader, flavor_offset, endian)?;
            if flavor.flavor != 0 && flavor.count != 0 {
                flavor_offset += LC_THREAD_FLAVOR_HEADER_SIZE + flavor.count as usize * size_of::<u32>();
                flavors.push(flavor);

                if flavor_offset < base_offset + cmdsize as usize {
                    continue;
                }
            }

            break;
        }

        Ok(LcThread { flavors })
    }
}

impl LcThreadFlavor {
    pub(super) fn parse(reader: &RcReader, base_offset: usize, endian: scroll::Endian) -> Result<Self> {
        let mut reader_mut = reader.borrow_mut();
        reader_mut.seek(SeekFrom::Start(base_offset as u64))?;

        let flavor: u32 = reader_mut.ioread_with(endian)?;
        let count: u32 = reader_mut.ioread_with(endian)?;

        let state_offset = reader_mut.stream_position()?;
        
        Ok(LcThreadFlavor { flavor, count, state_offset })
    }

    pub fn get_state_offset(&self) -> u64 {
        self.state_offset
    }
}

impl Debug for LcThreadFlavor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LcThreadFlavor")
            .field("flavor", &self.flavor)
            .field("count", &self.count)
            .finish()
    }
}