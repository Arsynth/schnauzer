use super::common;
use super::common::options::AddToOptions;
use super::common::Format;
use super::common::ObjectFilter;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::*;
use crate::*;
use colored::*;
use getopts::*;

static SUBCOMM_NAME: &str = "lc";

pub(super) struct LcHandler {
    pub(super) printer: Printer,
}

impl LcHandler {
    pub(super) fn new(printer: Printer) -> Self {
        LcHandler { printer }
    }
}

impl Handler for LcHandler {
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn description(&self) -> String {
        "Prints all load commands".to_string()
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

impl LcHandler {
    fn handle_load_commands(&self, commands: LoadCommandIterator, format: &Format) {
        for (idx, cmd) in commands.enumerate() {
            self.handle_command(cmd, idx, format);
        }
    }

    fn handle_command(&self, cmd: LoadCommand, index: usize, format: &Format) {
        if format.show_indices {
            self.printer.out_list_item_dash(0, index);
        }

        if format.short {
            self.printer.print_line(fmt_ext::load_command_to_string(cmd.cmd).yellow());
        } else {
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
    
            self.handle_command_variant(cmd.variant);
        }
    }

    fn handle_command_variant(&self, variant: LcVariant) {
        for field in variant.all_fields() {
            self.printer.print_string(" ");
            self.printer
                .out_default_colored_field(&field.name, &field.value, "\n");
        }
    }
}
