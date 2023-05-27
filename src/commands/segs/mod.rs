use super::common;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::AutoEnumFields;
use crate::*;
use colored::*;

mod confg;
use confg::*;

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

    fn handle_object(&self, object: ObjectType, _other_args: Vec<String>) -> Result<()> {
        match object {
            ObjectType::Fat(fat) => self.handle_fat(fat),
            ObjectType::MachO(macho) => self.handle_macho(macho, 0),
        }
        Ok(())
    }
}

impl SegsHandler {
    fn handle_fat(&self, fat: FatObject) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1);
            self.printer.print_line("")
        }
    }

    fn handle_arch(&self, arch: FatArch, index: usize) {
        let object = arch.object().unwrap();
        self.handle_macho(object, index);
    }

    fn handle_macho(&self, macho: MachObject, index: usize) {
        common::out_single_arch_title(&self.printer, &macho.header(), index);
        self.handle_load_commands(macho.load_commands_iterator());
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator) {
        let commands = commands.filter(|cmd| match cmd.variant {
            LcVariant::Segment32(_) | LcVariant::Segment64(_) => true,
            _ => false,
        });

        // Section index should start at 1
        let mut sect_index: usize = 1;
        for (index, cmd) in commands.enumerate() {
            match cmd.variant {
                LcVariant::Segment32(seg) => self.handle_segment_command(seg, index, &mut sect_index),
                LcVariant::Segment64(seg) => self.handle_segment_command(seg, index, &mut sect_index),
                _ => (),
            }
        }
    }

    fn handle_segment_command(&self, seg: LcSegment, seg_index: usize, sect_index: &mut usize) {
        self.printer.out_list_item_dash(0, seg_index);
        self.printer
            .print_colored_string("Segment (".bright_white());
        self.printer
            .out_default_colored_fields(seg.all_fields(), "");
        self.printer.print_colored_string("):\n".bright_white());

        for section in seg.sections_iterator() {
            self.handle_section(section, *sect_index);
            self.printer.print_line("");
            *sect_index += 1;
        }
    }

    fn handle_section(&self, section: Section, index: usize) {
        self.printer.print_string(format!(
            "  {} {}{} {} {} {}{}\n",
            "Section".bright_white(),
            "#".dimmed(),
            index.to_string().bright_white(),
            section.sectname.to_string().yellow(),
            "Segment".bright_white(),
            section.segname.to_string().yellow(),
            ":".bright_white()
        ));
        for field in section.all_fields().iter().skip(2) {
            self.printer.out_dashed_field(&field.name, &field.value, 1);
        }
    }
}

struct SegmentVariant {

}

struct SegmentPrinter {

}