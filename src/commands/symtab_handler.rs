use super::handler::*;
use super::Result;

pub(super) struct SymtabHandler {}

impl Handler for SymtabHandler {
    fn can_handle_with_args(&self) -> bool {
        false
    }

    fn handle_with_args(&self) -> Result<()> {
        todo!()
    }
}