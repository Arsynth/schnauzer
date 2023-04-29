//! `shnauzer` is a library for parsing Mach-O files
//! 
//! #References
//! 
//! <https://reverseengineering.stackexchange.com/questions/6356/what-is-a-fat-header>
//! <https://lowlevelbits.org/parsing-mach-o-files/>
//! <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html>
//! <https://opensource.apple.com/source/xnu/xnu-2050.18.24/EXTERNAL_HEADERS/mach-o/fat.h.auto.html>
//! <https://opensource.apple.com/source/cctools/cctools-698/libmacho/arch.c>
//! <https://opensource.apple.com/source/cctools/cctools-698/ld/>
//! <https://blog.xpnsec.com/building-a-mach-o-memory-loader-part-1/>
//! <https://github.com/aidansteele/osx-abi-macho-file-format-reference>

pub mod result;
pub mod types;
pub mod constants;

mod reader;

use std::path::Path;

use self::result::Result;
pub use types::*;

use std::cell::RefCell;
use std::rc::Rc;
use reader::Reader;

type RcCell<T> = Rc<RefCell<T>>;

use reader::RcReader;

pub struct Parser {
    reader: RcReader
}

impl Parser {
    pub fn build(path: &Path) -> Result<Parser> {
        let reader = Reader::build(path)?;
        Ok(Parser {
            reader
        })
    }

    pub fn parse(self) -> Result<ObjectType> {
        ObjectType::parse(self.reader.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_output() {
        let path = Path::new("testable/cat");
        let parser = Parser::build(path).unwrap();
        let obj = parser.parse().unwrap();
        println!("{:#?}", obj);
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
            assert_eq!(item.cpu_type, 16777223);
            assert_eq!(item.cpu_subtype, 3);
            assert_eq!(item.offset, 16384);
            assert_eq!(item.size, 70080);
            assert_eq!(item.align, 14);
        }

        {
            let item = &arch_items[1];
            assert_eq!(item.cpu_type, 16777228);
            assert_eq!(item.cpu_subtype, constants::CPU_SUBTYPE_LIB64 | 0x00000002);
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
            assert_eq!(arch.cpu_type, arch.object().unwrap().header().cpu_type);
            assert_eq!(arch.cpu_subtype, arch.object().unwrap().header().cpu_subtype);
        }
    }
    
}
