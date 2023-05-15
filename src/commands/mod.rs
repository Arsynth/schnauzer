mod handler;
mod default_handler;
mod symtab_handler;

use super::output::Printer;
use super::result::*;

use handler::*;
use default_handler::*;
use symtab_handler::*;


pub fn handle_with_args() -> Result<()> {
    for handler in available_handlers().iter() {
        if handler.can_handle_with_args() {
            return handler.handle_with_args();
        }
    }

    DefaultHandler::new(Printer {}).handle_with_args()?;

    Ok(())
}

fn available_handlers() -> Vec<Box<dyn Handler>> {
    vec![Box::new(SymtabHandler {})]
}
