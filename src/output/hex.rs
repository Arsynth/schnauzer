use crate::result::Result;
use kex::*;
use std::io::{stdout};

use crate::Section;

pub(crate) fn dump_section(sect: &Section) -> Result<()> {
    let fmt = Formatters::new(
        AddressFormatter::new(16),
        ByteFormatter::new(),
        CharFormatter::new(),
    );
    let config = Config::new(fmt, 16, 4, ("|".to_string(), '|'.to_string()));
    let mut out = Printer::new(Box::new(stdout()), sect.addr.0 as usize, config);
    let result = sect.read_data_to(&mut out);
    out.finish();
    result
}
