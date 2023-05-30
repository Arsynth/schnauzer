use super::common;
use super::common::options::AddToOptions;
use super::handler;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::AutoEnumFields;
use crate::*;
use colored::*;

mod confg;
use confg::*;
use getopts::Options;

static SUBCOMM_NAME: &str = "segs";

pub(super) struct SegsHandler {
    printer: Printer,
}

impl SegsHandler {
    pub(super) fn new(printer: Printer) -> Self {
        Self { printer }
    }
}

impl Handler for SegsHandler {
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let mut opts = Options::new();
        self.accepted_option_items().add_to_opts(&mut opts);
        let config = Config::build(&mut opts, &other_args)?;

        match object {
            ObjectType::Fat(fat) => self.handle_fat(fat, &config),
            ObjectType::MachO(macho) => self.handle_macho(macho, 0, &config),
        }
        Ok(())
    }

    fn accepted_option_items(&self) -> Vec<common::options::OptionItem> {
        let mut items = handler::default_option_items();
        items.append(&mut Config::option_items());
        items
    }
}

impl SegsHandler {
    fn handle_fat(&self, fat: FatObject, config: &Config) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1, &config);
            self.printer.print_line("")
        }
    }

    fn handle_arch(&self, arch: FatArch, index: usize, config: &Config) {
        let object = arch.object().unwrap();
        self.handle_macho(object, index, config);
    }

    fn handle_macho(&self, macho: MachObject, index: usize, config: &Config) {
        common::out_single_arch_title(&self.printer, &macho.header(), index, config.format.short);
        self.handle_load_commands(macho.load_commands_iterator(), config);
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator, config: &Config) {
        let commands = commands.filter(|cmd| match cmd.variant {
            LcVariant::Segment32(_) | LcVariant::Segment64(_) => true,
            _ => false,
        });

        // Section index should start at 1
        let mut sect_index: usize = 1;
        for (index, cmd) in commands.enumerate() {
            match cmd.variant {
                LcVariant::Segment32(seg) | LcVariant::Segment64(seg) => {
                    self.handle_segment_command(seg, index, &mut sect_index, config)
                }
                _ => (),
            }
        }
    }

    fn handle_segment_command(
        &self,
        seg: LcSegment,
        seg_index: usize,
        sect_index: &mut usize,
        config: &Config,
    ) {
        if config.show_segs {
            let seg_printer = SegmentPrinter {
                printer: &self.printer,
                segment: &seg,
                short: config.format.short,
                show_indices: config.format.show_indices,
                index: seg_index,
            };
            seg_printer.print();
        }
        
        if config.show_sects {
            for section in seg.sections_iterator() {
                let sect_printer = SectionPrinter {
                    printer: &self.printer,
                    section: &section,
                    short: config.format.short,
                    show_indices: config.format.show_indices,
                    index: *sect_index,
                    segment_index: seg_index,
                };
                sect_printer.print();
                if !config.format.short {
                    self.printer.print_line("");
                }
                *sect_index += 1;
            }
        }
    }
}

struct SegmentPrinter<'a> {
    printer: &'a Printer,
    segment: &'a LcSegment,
    index: usize,

    short: bool,
    show_indices: bool,
}

impl<'a> SegmentPrinter<'a> {
    fn print(&self) {
        if self.show_indices {
            self.printer.out_list_item_dash(0, self.index);
        }
        if self.short {
            self.printer.print_line(&self.segment.segname.to_string().yellow());
        } else {
            self.printer
                .print_colored_string("Segment (".bright_white());
            self.printer
                .out_default_colored_fields(self.segment.all_fields(), "");
            self.printer.print_colored_string("):\n".bright_white());
        }
    }
}

struct SectionPrinter<'a> {
    printer: &'a Printer,
    section: &'a Section,
    index: usize,
    segment_index: usize,

    short: bool,
    show_indices: bool,
}

impl<'a> SectionPrinter<'a> {
    fn print(&self) {
        if self.show_indices {
            if self.short {
                self.printer
                    .out_index_path_dash(1, vec![self.segment_index, self.index]);
            } else {
                self.printer.print_string(format!(
                    " {} {}{} ",
                    "Section".bright_white(),
                    "#".dimmed(),
                    self.index.to_string().bright_white(),
                ))
            }
        }

        if self.short {
            self.printer.print_line(vec![
                self.section.segname.to_string().yellow().to_string(),
                self.section.sectname.to_string().yellow().to_string(),
            ].join(" "));
        } else {
            self.printer.print_string(format!(
                "{} {} {}{}\n",
                self.section.sectname.to_string().yellow(),
                "Segment".bright_white(),
                self.section.segname.to_string().yellow(),
                ":".bright_white()
            ));
            for field in self.section.all_fields().iter().skip(2) {
                self.printer.out_dashed_field(&field.name, &field.value, 1);
            }
        }
    }
}
