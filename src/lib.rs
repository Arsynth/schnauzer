//! `shnauzer` is a library for parsing Mach-O files
//! 
//! #References
//! 

pub mod result;
pub mod types;
pub mod constants;
pub mod fmt_ext;
pub mod auto_enum_fields;
pub mod output;
pub mod commands;

mod reader;

use std::path::Path;

use self::result::Result;
pub use types::*;

use std::cell::RefCell;
use std::rc::Rc;
use reader::Reader;

type RcCell<T> = Rc<RefCell<T>>;

use reader::RcReader;

/// Topmost struct in the library.
/// Reads file in lazy manner (doesn't load all contents to memory).
pub struct Parser {
    reader: RcReader
}

impl Parser {
    /// Provide a valid path to binary, library, object file, e.t.c.. 
    /// Full list of mach-o files you can find there - [filetype_constants].
    /// For example: `/bin/cat`.
    pub fn build(path: &Path) -> Result<Parser> {
        let reader = Reader::build(path)?;
        Ok(Parser {
            reader
        })
    }

    /// Returns appropriate object - [FatObject] or [MachObject]
    pub fn parse(self) -> Result<ObjectType> {
        ObjectType::parse(self.reader.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu_constants::CPU_SUBTYPE_LIB64;

    use super::*;

    #[test]
    fn test_can_output() {
        let path = Path::new("testable/cat");
        let parser = Parser::build(path).unwrap();
        let obj = parser.parse().unwrap();
        println!("{:#?}", obj);
    }
    
    #[test]
    fn test_basic_parsing_stability() {
        let path = Path::new("testable/cat");
        let parser = Parser::build(path).unwrap();
        let obj = parser.parse().unwrap();

        let first = format!("{:#?}", obj);
        let second = format!("{:#?}", obj);
        
        assert_eq!(first, second, "Somewhere invalid offset used while parsing!");
    }

    #[test]
    fn test_binary() {
        let path = Path::new("testable/cat");
        let parser = Parser::build(path).unwrap();
        let obj = parser.parse().unwrap();
        
        let fat_header = if let ObjectType::Fat(f) = obj {
            f
        } else {
            panic!("Expected fat header, got {:#?}", obj);
        };

        let arch_items: Vec<FatArch> = fat_header.arch_iterator().collect();
        assert_eq!(arch_items.len(), 2, "Should be only two architectures");

        {
            let item = &arch_items[0];
            assert_eq!(item.cputype.0, 16777223);
            assert_eq!(item.cpusubtype.0, 3);
            assert_eq!(item.offset, 16384);
            assert_eq!(item.size, 70080);
            assert_eq!(item.align, 14);
        }

        {
            let item = &arch_items[1];
            assert_eq!(item.cputype.0, 16777228);
            assert_eq!(item.cpusubtype.0, CPU_SUBTYPE_LIB64 | 0x00000002);
            assert_eq!(item.offset, 98304);
            assert_eq!(item.size, 53488);
            assert_eq!(item.align, 14);
        }
    }

    
    #[test]
    fn test_arch_with_header_consistency() {
        let path = Path::new("testable/cat");
        let parser = Parser::build(path).unwrap();
        let obj = parser.parse().unwrap();
        
        let fat_header = if let ObjectType::Fat(f) = obj {
            f
        } else {
            panic!("Expected fat header, got {:#?}", obj);
        };

        for arch in fat_header.arch_iterator() {
            assert_eq!(arch.cputype.0, arch.object().unwrap().header().cputype.0);
            assert_eq!(arch.cpusubtype.0, arch.object().unwrap().header().cpusubtype.0);
        }
    }
    
}
