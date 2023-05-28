use super::handler::*;
use super::Printer;
use super::Result;
use crate::*;
use super::common;

static SUBCOMM_NAME: &str = "rpaths";

pub(super) struct RpathsHandler {
    printer: Printer,
}

impl RpathsHandler {
    pub(super) fn new(printer: Printer) -> Self {
        Self { printer }
    }
}

impl Handler for RpathsHandler {
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

impl RpathsHandler {
    fn handle_fat(&self, fat: FatObject) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1);
            self.printer.print_line("");
        }
    }

    fn handle_arch(&self, arch: FatArch, index: usize) {
        let object = arch.object().unwrap();
        self.handle_macho(object, index);
    }

    fn handle_macho(&self, macho: MachObject, index: usize) {
        common::out_single_arch_title(&self.printer, &macho.header(), index, false);
        self.handle_load_commands(macho.load_commands_iterator());
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator) {
        let commands = commands.flat_map(|cmd| match cmd.variant {
            LcVariant::Rpath(rpath) => Some(rpath),
            _ => None,
        });
        for (index, cmd) in commands.enumerate() {
            self.handle_rpath_command(cmd, index);
        }
    }

    fn handle_rpath_command(&self, cmd: LcRpath, index: usize) {
        self.printer.out_list_item_dash(0, index);
        self.printer.print_line(common::colored_path_string(cmd.path));
    }
}