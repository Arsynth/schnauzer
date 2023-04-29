use scroll::IOread;
use std::fmt::{Debug};

use super::result::Result;
use super::reader::RcReader;

pub mod magic;
pub use magic::Magic;

pub mod fat_object;
pub use fat_object::*;

pub mod fat_arch;
pub use fat_arch::*;

pub mod mach_object;
pub use mach_object::*;

pub mod mach_header;
pub use mach_header::*;

pub mod load_command;
pub use load_command::*;

pub mod segment;
pub use segment::*;

pub(crate) use super::constants;

#[derive(Debug)]
pub enum ObjectType {
    Fat(FatObject),
    MachO(MachObject),
}

impl ObjectType {
    pub(super) fn parse(reader: RcReader) -> Result<ObjectType> {
        let magic = reader.borrow_mut().ioread_with::<u32>(scroll::BE)?;
        let magic: Magic = magic.try_into()?;
        if magic.is_fat() {
            let header = FatObject::parse(reader.clone())?;
            Ok(ObjectType::Fat(header))
        } else {
            let header = MachObject::parse(reader.clone(), 0)?;
            Ok(ObjectType::MachO(header))
        }
    }
}

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
