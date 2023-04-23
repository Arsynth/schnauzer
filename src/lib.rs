//! `shnauzer` is a library for parsing Mach-O files
//! 
//! #References
//! 
//! <https://reverseengineering.stackexchange.com/questions/6356/what-is-a-fat-header>
//! <https://lowlevelbits.org/parsing-mach-o-files/>
//! <https://opensource.apple.com/source/xnu/xnu-4570.71.2/EXTERNAL_HEADERS/mach-o/loader.h.auto.html>
//! <https://opensource.apple.com/source/xnu/xnu-2050.18.24/EXTERNAL_HEADERS/mach-o/fat.h.auto.html>
//! <https://opensource.apple.com/source/cctools/cctools-698/libmacho/arch.c>

pub mod result;
pub mod types;
pub mod constants;

use self::result::Result;
pub use types::*;

pub struct Parser;

impl Parser {
    pub fn parse(buf: &[u8]) -> Result<ObjectType> {
        ObjectType::parse(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary() {
        let path = "testable/cat";
        let buf = std::fs::read(path).unwrap();

        let obj = Parser::parse(&buf).unwrap();
        let fat_header = if let ObjectType::Fat(f) = obj {
            f
        } else {
            panic!("Expected fat header, got {:#?}", obj);
        };

        let arch1 = FatArch {
            buf: &buf,
            cpu_type: 16777223,
            cpu_subtype: 3,
            offset: 16384,
            size: 70080,
            align: 14,
        };

        let arch2 = FatArch {
            buf: &buf,
            cpu_type: 16777228,
            cpu_subtype: constants::CPU_SUBTYPE_LIB64 | 0x00000002,
            offset: 98304,
            size: 53488,
            align: 14,
        };

        let arch_items: Vec<FatArch> = fat_header.arch_iterator().collect();
        assert_eq!(arch_items.len(), 2, "Should be only two architectures");

        assert_eq!(arch1, arch_items[0]);
        assert_eq!(arch2, arch_items[1]);
    }

    #[test]
    fn test_arch_with_header_consistency() {
        let path = "testable/cat";
        let buf = std::fs::read(path).unwrap();

        let obj = Parser::parse(&buf).unwrap();
        let fat_header = if let ObjectType::Fat(f) = obj {
            f
        } else {
            panic!("Expected fat header, got {:#?}", obj);
        };

        for arch in fat_header.arch_iterator() {
            assert_eq!(arch.cpu_type, arch.mach_header().unwrap().cpu_type);
            assert_eq!(arch.cpu_subtype, arch.mach_header().unwrap().cpu_subtype);
        }
    }
}
