use super::Printer;
use super::Result;
use crate::auto_enum_fields::*;
use crate::*;
use colored::*;

pub(super) struct DefaultHandler {
    printer: Printer,
}

impl DefaultHandler {
    pub(super) fn new(printer: Printer) -> Self {
        DefaultHandler { printer }
    }
}

impl DefaultHandler {
    pub(super) fn handle_object(&self, object: ObjectType, _other_args: Vec<String>) -> Result<()> {
        self.handle_object_type(object);
        Ok(())
    }
}

impl DefaultHandler {
    fn handle_object_type(&self, obj: ObjectType) {
        match obj {
            ObjectType::Fat(fat) => self.handle_fat(fat),
            ObjectType::MachO(macho) => self.handle_macho(macho, false),
        }
    }

    fn handle_fat(&self, fat: FatObject) {
        for arch in fat.arch_iterator() {
            self.handle_arch(arch);
        }
    }

    fn handle_arch(&self, arch: FatArch) {
        self.printer.print_line(format!("{}", "Fat arch:".bold().bright_white()));

        for field in arch.all_fields() {
            self.printer.out_dashed_field(&field.name, &field.value, 0);
        }
        self.printer
            .out_dashed_field("Mach header", "", 0);

        self.handle_macho(arch.object().unwrap(), true);
    }

    fn handle_macho(&self, macho: MachObject, nested: bool) {
        let level = match nested {
            true => 2,
            false => 1,
        };

        let h = macho.header();
        for field in h.all_fields() {
            self.printer
                .out_dashed_field(&field.name, &field.value, level);
        }
        self.printer
            .out_dashed_field("Load commands", "", level);

        self.handle_load_commands(macho.load_commands_iterator(), level + 1);
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator, level: usize) {
        for (index, cmd) in commands.enumerate() {
            self.printer.out_list_item_dash(level, index);
            self.printer.out_field(
                "cmd".bright_white(),
                fmt_ext::load_command_to_string(cmd.cmd).yellow(),
                " ",
            );
            self.printer.out_field(
                "cmdsize".bright_white(),
                cmd.cmdsize.to_string().yellow(),
                "\n",
            );

            self.handle_command_variant(cmd.variant, level + 1);
        }
    }

    fn handle_command_variant(&self, variant: LcVariant, level: usize) {
        for field in variant.all_fields() {
            self.printer.out_field_dash(level);
            self.printer
                .out_default_colored_field(&field.name, &field.value, "\n");
        }
        match variant {
            LcVariant::Segment32(seg) => self.handle_segment_command(seg, level),
            LcVariant::Segment64(seg) => self.handle_segment_command(seg, level),
            LcVariant::Thread(thread) => self.handle_thread_flavor(thread, level),
            _ => (),
        }
    }

    fn handle_segment_command(&self, seg: LcSegment, level: usize) {
        if seg.nsects > 0 {
            self.printer
                .out_dashed_field("Sections", "", level);
        }
        for section in seg.sections_iterator() {
            self.handle_section(section, level + 1);
            self.printer.out_tile(level + 1);
        }
    }

    fn handle_section(&self, section: Section, level: usize) {
        for field in section.all_fields() {
            self.printer
                .out_dashed_field(&field.name, &field.value, level + 1);
        }
    }

    fn handle_thread_flavor(&self, thread: LcThread, level: usize) {
        self.printer
            .out_dashed_field("Flavors", "", level);

        for thread_flavor in thread.flavor_iterator() {
            for field in thread_flavor.all_fields() {
                self.printer
                    .out_dashed_field(&field.name, &field.value, level + 1);
            }

            self.printer.out_tile(level + 1);
        }
    }
}
