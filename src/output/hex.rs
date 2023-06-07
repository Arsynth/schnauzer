use crate::result::Result;
use kex::*;
use std::io::stdout;

use crate::Section;

pub(crate) fn dump_section(sect: &Section) -> Result<()> {
    let fmt = Formatters::new(
        AddressFormatter::new(16),
        EndianFormatter::new(sect.endian),
        CharFormatter::new(),
    );
    let config = StrictConfig::new(fmt, 4, Default::default());
    let mut wrapper =
        StrictPrinter::<_, _, _, _, 4>::new(Box::new(stdout()), sect.addr.0 as usize, config);
    let result = sect.read_data_to(&mut wrapper);
    wrapper.finish();
    result
}

pub struct EndianFormatter {
    endian: scroll::Endian,
}

impl EndianFormatter {
    pub fn new(endian: scroll::Endian) -> Self {
        Self { endian }
    }
}

impl ByteFormatting for EndianFormatter {
    fn format(&mut self, bytes: &[u8]) -> String {
        let transform = |b: &u8| format!("{:02x}", b);

        let strs: Vec<String> = match self.endian {
            scroll::Endian::Little => bytes.iter().rev().map(transform).collect(),
            scroll::Endian::Big => bytes.iter().map(transform).collect(),
        };

        // let strs: Vec<String> = iter;
        strs.join("")
    }

    fn padding_string(&mut self, byte_count: usize) -> String {
        "..".repeat(byte_count)
    }
}
