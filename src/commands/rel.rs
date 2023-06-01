use super::common;
use super::common::options::AddToOptions;
use super::common::ObjectFilter;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::*;
use colored::*;
use getopts::*;

static SUBCOMM_NAME: &str = "rel";

pub(super) struct RelHandler {
    pub(super) printer: Printer,
}

impl RelHandler {
    pub(super) fn new(printer: Printer) -> Self {
        RelHandler { printer }
    }
}

impl Handler for RelHandler {
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn description(&self) -> String {
        "Prints relocation entries".to_string()
    }

    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let mut opts = Options::new();
        self.accepted_option_items().add_to_opts(&mut opts);

        let filter = ObjectFilter::build(&mut opts, &other_args)?;

        let objects = &filter.get_objects(object);
        let out_arch = objects.len() > 1;
        for (idx, obj) in objects.iter().enumerate() {
            if out_arch {
                common::out_single_arch_title(&self.printer, &obj.header(), idx, false);
            }
            self.handle_load_commands(obj.load_commands_iterator());
        }

        Ok(())
    }
}

impl RelHandler {
    fn handle_load_commands(&self, commands: LoadCommandIterator) {
        let commands = commands.filter(|cmd| match cmd.variant {
            LcVariant::Segment32(_) | LcVariant::Segment64(_) => true,
            _ => false,
        });
        for cmd in commands {
            match cmd.variant {
                LcVariant::Segment32(seg) | LcVariant::Segment64(seg) => {
                    self.handle_segment_command(seg);
                }
                _ => (),
            }
        }
    }

    fn handle_segment_command(&self, seg: LcSegment) {
        use output::table::FixedTabLine;

        let line: FixedTabLine<7> = FixedTabLine::new([9, 6, 7, 7, 5, 10, 16]);

        for section in seg.sections_iterator().filter(|s| s.nreloc > 0) {
            self.printer.print_strings(
                vec![
                    section.segname.to_string().green(),
                    section.sectname.to_string().green(),
                ],
                " ",
            );

            self.printer.print_line(
                format!(" ({} entries)", section.nreloc.to_string().blue()).bright_white(),
            );

            line.print_line(["address", "pcrel", "length", "extern", "type", "scattered", "symbolnum/value"], vec![Color::White]);
            for reloc in section.relocations_iterator() {
                line.print_line([
                    format!("{:08x}", reloc.r_address),
                    reloc.r_pcrel().to_string(),
                    reloc.r_length().to_string(),
                    reloc.r_extern().to_string(),
                    reloc.r_type().to_string(),
                    reloc.is_scattered().to_string(),
                    reloc.r_symbolnum().to_string(),
                ],
                vec![Color::Red, Color::Yellow]
            );
            }
        }
    }
}
