use crate::result::Result;
use colored::Colorize;
use kex::*;
use std::io::stdout;

use crate::Section;

pub(crate) fn dump_section(sect: &Section) -> Result<()> {
    let config = Config::new(
        Some(AddressFormatter::new(
            AddressStyle::Hex(16),
            Separators::new("", &": ".green().to_string()),
        )),
        ByteFormatter::new(
            Groupping::RepeatingGroup(Group::new(4, " "), 4),
            sect.endian.is_little(),
            Default::default(),
        ),
        Some(CharFormatter::new(".".dimmed().to_string(), Default::default())),
    );
    
    let mut printer = Printer::new(stdout(), sect.addr.0 as usize, config);

    let result = sect.read_data_to(&mut printer);
    printer.finish();

    result
}
