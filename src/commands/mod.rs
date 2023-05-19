mod default;
mod handler;
mod syms;
mod rpaths;
mod dylibs;
mod segs;
mod fat;
mod headers;

mod common;
mod helpers;

use super::output::Printer;
use super::result::*;

use default::*;
use handler::*;
use syms::*;
use rpaths::*;
use dylibs::*;
use segs::*;
use fat::*;
use headers::*;

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
    let printer = Printer {};
    vec![
        Box::new(SymsHandler::new(printer.clone())),
        Box::new(RpathsHandler::new(printer.clone())),
        Box::new(DylibsHandler::new(printer.clone())),
        Box::new(SegsHandler::new(printer.clone())),
        Box::new(ArchsHandler::new(printer.clone())),
        Box::new(HeadersHandler::new(printer.clone())),
    ]
}
