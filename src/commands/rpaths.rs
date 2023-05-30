use getopts::Options;

use super::common::Format;
use super::common::ObjectFilter;
use super::common::options::*;
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

impl RpathsHandler {
    fn handle_load_commands(&self, commands: LoadCommandIterator, format: &Format) {
        let commands = commands.flat_map(|cmd| match cmd.variant {
            LcVariant::Rpath(rpath) => Some(rpath),
            _ => None,
        });
        for (index, cmd) in commands.enumerate() {
            if format.show_indices {
                self.printer.out_list_item_dash(0, index);
            }
            self.printer.print_line(common::colored_path_string(cmd.path));
        }
    }
}