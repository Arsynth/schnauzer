use super::common;
use super::common::options::AddToOptions;
use super::common::ObjectFilter;
use super::handler;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::*;
use colored::*;

mod config;
use config::*;
use getopts::Options;

static SUBCOMM_NAME: &str = "data";

pub(super) struct DataHandler {
    printer: Printer,
}

impl DataHandler {
    pub(super) fn new(printer: Printer) -> Self {
        Self { printer }
    }
}

impl Handler for DataHandler {
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn description(&self) -> String {
        "Prints hex dump".to_string()
    }

    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let mut opts = Options::new();
        self.accepted_option_items().add_to_opts(&mut opts);
        let config = Config::build(&mut opts, &other_args)?;
        let filter = ObjectFilter::build(&mut opts, &other_args)?;

        let objects = &filter.get_objects(object);
        let out_arch = objects.len() > 1;
        for (idx, obj) in objects.iter().enumerate() {
            if out_arch {
                common::out_single_arch_title(
                    &self.printer,
                    &obj.header(),
                    idx,
                    false,
                );
            }
            self.handle_load_commands(obj.load_commands_iterator(), &config);
        }

        Ok(())
    }

    fn accepted_option_items(&self) -> Vec<common::options::OptionItem> {
        let mut items = handler::default_option_items();
        items.append(&mut Config::option_items());
        items
    }
}

impl DataHandler {
    fn handle_load_commands(&self, commands: LoadCommandIterator, config: &Config) {
        let segs = commands.filter_map(|cmd| match cmd.variant {
            LcVariant::Segment32(s) | LcVariant::Segment64(s) => {
                if s.segname.to_string() == config.seg {
                    Some(s)
                } else {
                    None
                }
            }
            _ => None,
        });

        let sections = segs.filter_map(|seg| {
            seg.sections_iterator()
                .find(|s| s.sectname.to_string() == config.sect)
        });

        for sect in sections {
            self.handle_section(sect);
        }
    }

    fn handle_section(&self, sect: Section) {
        use crate::output::hex::*;
        println!("{} {}", sect.segname.to_string().yellow(), sect.sectname.to_string().yellow());

        _ = dump_section(&sect);
        println!("")
    }
}
