use super::common;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::AutoEnumFields;
use crate::*;
use colored::*;

mod confg;
use confg::*;

use super::EXEC_NAME;

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
    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let filter = Filter::build(other_args);
        match object {
            ObjectType::Fat(fat) => self.handle_fat(fat, &filter),
            ObjectType::MachO(macho) => self.handle_macho(macho, 0, &filter),
        }
        Ok(())
    }

    fn options(&self) -> getopts::Options {
        confg::Filter::options()
    }

    fn help_string(&self) -> String {
        confg::help_string()
    }
}

impl SegsHandler {
    fn handle_fat(&self, fat: FatObject, filter: &Filter) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1, &filter);
            self.printer.print_line("")
        }
    }

    fn handle_arch(&self, arch: FatArch, index: usize, filter: &Filter) {
        let object = arch.object().unwrap();
        self.handle_macho(object, index, filter);
    }

    fn handle_macho(&self, macho: MachObject, index: usize, filter: &Filter) {
        common::out_single_arch_title(&self.printer, &macho.header(), index, filter.short);
        self.handle_load_commands(macho.load_commands_iterator(), filter);
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator, filter: &Filter) {
        let commands = commands.filter(|cmd| match cmd.variant {
            LcVariant::Segment32(_) | LcVariant::Segment64(_) => true,
            _ => false,
        });

        // Section index should start at 1
        let mut sect_index: usize = 1;
        for (index, cmd) in commands.enumerate() {
            match cmd.variant {
                LcVariant::Segment32(seg) | LcVariant::Segment64(seg) => {
                    self.handle_segment_command(seg, index, &mut sect_index, filter)
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
        filter: &Filter,
    ) {
        match filter.mode {
            Mode::Both | Mode::SegmentsOnly => {
                let seg_printer = SegmentPrinter {
                    printer: &self.printer,
                    segment: &seg,
                    short: filter.short,
                    show_indices: filter.show_indices,
                    index: seg_index,
                };
                seg_printer.print();
            },
            Mode::SectionsOnly => (),
        }
        
        match filter.mode {
            Mode::Both | Mode::SectionsOnly => {
                for section in seg.sections_iterator() {
                    let sect_printer = SectionPrinter {
                        printer: &self.printer,
                        section: &section,
                        short: filter.short,
                        show_indices: filter.show_indices,
                        index: *sect_index,
                        segment_index: seg_index,
                    };
                    sect_printer.print();
                    if !filter.short {
                        self.printer.print_line("");
                    }
                    *sect_index += 1;
                }
            },
            Mode::SegmentsOnly => (),
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
                self.section.sectname.to_string().yellow().to_string(),
                self.section.segname.to_string().yellow().to_string(),
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
