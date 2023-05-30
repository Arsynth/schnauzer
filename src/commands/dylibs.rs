use super::common;
use super::common::Format;
use super::common::ObjectFilter;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::*;
use colored::*;
use super::common::options::*;
use getopts::*;

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
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let mut opts = Options::new();
        self.accepted_option_items().add_to_opts(&mut opts);

        let format = &Format::build(&mut opts, &other_args)?;
        let filter = ObjectFilter::build(&mut opts, &other_args)?;

        let objects = &filter.get_objects(object);
        let out_arch = objects.len() > 1;
        for (idx, obj) in objects.iter().enumerate() {
            if out_arch {
                common::out_single_arch_title(&self.printer, &obj.header(), idx, format.short);
            }
            self.handle_load_commands(obj.load_commands_iterator(), format);
        }

        Ok(())
    }

    fn accepted_option_items(&self) -> Vec<common::options::OptionItem> {
        let mut result = default_option_items();
        result.append(&mut Format::option_items());
        result
    }
}

impl DylibsHandler {
    fn handle_load_commands(&self, commands: LoadCommandIterator, format: &Format) {
        let commands = commands.flat_map(|cmd| match cmd.variant {
            LcVariant::LoadDylib(dylib) => Some(dylib),
            _ => None,
        });
        for (index, cmd) in commands.enumerate() {
            self.handle_dylib_command(cmd, index, format);
        }
    }

    fn handle_dylib_command(&self, dylib: LcDylib, index: usize, format: &Format) {
        if format.show_indices {
            self.printer.out_list_item_dash(0, index);
        }

        let name = common::colored_path_string(dylib.name);
        self.printer.print_string(name);

        if !format.short {
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
        }

        self.printer.print_line("");
    }
}
