use getopts::Options;
pub use std::env::Args;

use crate::ObjectType;

use super::Result;

use super::common::*;

pub(super) trait Handler {
    fn can_handle_with_name(&self, name: &str) -> bool;
    /// Function takes remainder of args, that it, without exec and subcommand names
    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()>;

    fn options(&self) -> Options {
        default_options()
    }

    fn help_string(&self) -> String {
        default_help_string()
    }
}

pub(super) fn default_options() -> Options {
    let mut opts = Options::new();
    opts.optopt(PATH_OPT_SHORT, PATH_OPT_LONG, "Path to file", "FILE");
    opts.optflag(HELP_FLAG_SHORT, HELP_FLAG_LONG, "");
    opts
}

pub(super) fn default_help_string() -> String {
    HELP_STRING.to_string()
}