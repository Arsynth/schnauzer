pub use std::env::Args;
use crate::ObjectType;

use super::Result;

pub(super) trait Handler {
    fn can_handle_with_name(&self, name: &str) -> bool;
    /// Function takes remainder of args, that it, without exec and subcommand names
    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()>;
}
