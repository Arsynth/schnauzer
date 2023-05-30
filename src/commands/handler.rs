use getopts::Options;
use options::*;
pub use std::env::Args;

use crate::ObjectType;

use super::Result;

use super::common::*;

pub(super) trait Handler {
    fn command_name(&self) -> String;
    fn description(&self) -> String;

    fn can_handle_with_name(&self, name: &str) -> bool;
    /// Function takes remainder of args, that it, without exec and subcommand names
    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()>;

    fn accepted_option_items(&self) -> Vec<OptionItem> {
        default_option_items()
    }
}

pub(crate) fn default_options() -> Options {
    let mut opts = Options::new();
    default_option_items().add_to_opts(&mut opts);
    opts
}

pub(crate) fn default_option_items() -> Vec<OptionItem> {
    let mut result = vec![
        OptionItem {
            option_type: OptionType::Arg(IsRequired(false)),
            name: OptionName::ShortLong(PATH_OPT_SHORT.to_string(), PATH_OPT_LONG.to_string()),
            description: "Path to file".to_string(),
            hint: "FILE".to_string(),
        },
        OptionItem {
            option_type: OptionType::Flag(IsRequired(false)),
            name: OptionName::ShortLong(HELP_FLAG_SHORT.to_string(), HELP_FLAG_LONG.to_string()),
            description: "Help".to_string(),
            hint: "".to_string(),
        },
    ];

    result.append(&mut ObjectFilter::option_items());

    result
}
