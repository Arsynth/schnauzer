use super::common;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::*;
use colored::*;

static SUBCOMM_NAME: &str = "dylibs";

pub(super) struct DylibsHandler {
    pub(super) printer: Printer,
}

impl DylibsHandler {
    pub(super) fn new(printer: Printer) -> Self {
        DylibsHandler { printer }
    }
}

impl Handler for DylibsHandler {
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

impl DylibsHandler {
    fn handle_fat(&self, fat: FatObject) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1);
            println!("");
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
        let commands = commands.flat_map(|cmd| match cmd.variant {
            LcVariant::LoadDylib(dylib) => Some(dylib),
            _ => None,
        });
        for (index, cmd) in commands.enumerate() {
            self.handle_dylib_command(cmd, index);
        }
    }

    fn handle_dylib_command(&self, dylib: LcDylib, index: usize) {
        self.printer.out_list_item_dash(0, index);
        let name = common::colored_path_string(dylib.name);
        self.printer.print_string(name);

        self.printer.print_colored_string(" (".bright_white());
        self.printer.out_default_colored_field(
            "Timestamp",
            &dylib.timestamp.to_string(),
            ", ",
        );
        self.printer.out_default_colored_field(
            "Current version",
            &dylib.current_version.to_string(),
            ", ",
        );
        self.printer.out_default_colored_field(
            "Compatibility version",
            &dylib.compatibility_version.to_string(),
            "",
        );
        self.printer.print_colored_string(")".bright_white());
        self.printer.print_line("".to_string());
    }
}
