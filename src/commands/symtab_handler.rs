use super::handler::*;
use super::Result;
pub use std::env::Args;
use std::{path::Path};
use super::Printer;
use crate::*;
use crate::auto_enum_fields::*;
use colored::*;

pub(super) struct SymtabHandler {}

impl Handler for SymtabHandler {
    fn can_handle_with_args(&self) -> bool {
        false
    }

    fn handle_with_args(&self) -> Result<()> {
        todo!()
    }
}