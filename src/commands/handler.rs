pub use std::env::Args;
use super::Result;

pub(super) trait Handler {
    fn can_handle_with_args(&self) -> bool;
    fn handle_with_args(&self) -> Result<()>;
}
