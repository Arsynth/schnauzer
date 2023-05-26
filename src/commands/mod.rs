mod default;
mod dylibs;
mod fat;
mod handler;
mod headers;
mod rpaths;
mod segs;
mod syms;

mod common;
mod helpers;

use super::output::Printer;
use super::result::*;

use colored::Colorize;
use default::*;
use dylibs::*;
use fat::*;
use getopts::{Options};
use handler::*;
use headers::*;
use rpaths::*;
use segs::*;
use syms::*;

pub fn handle_with_args() -> Result<()> {
    const PATH_OPT: &str = "p";

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        helpers::exit_with_help_string();
    } else if requires_help(args.clone()) {
        helpers::exit_normally_with_help_string();
    }

    let handler = matched_handler(&args[1]);
    let args = match handler {
        Some(_) => match args.get(2..) {
            Some(args) => args,
            None => {
                helpers::exit_with_help_string();
            },
        },
        None => match args.get(1..) {
            Some(args) => args,
            None => {
                helpers::exit_with_help_string();
            },
        },
    };

    if args.len() == 0 {
        helpers::exit_with_help_string();
    }

    let mut opts = Options::new();
    opts.optopt(
        PATH_OPT,
        "path",
        "Path to file",
        "FILE",
    );

    let path = match opts.parse(args.clone()) {
        Ok(m) => match m.opt_str(PATH_OPT) {
            Some(path) => path,
            None => args[0].clone(),
        }
        Err(f) => {
            eprint!("{}\n\n", f.to_string());
            helpers::exit_with_help_string()
        }
    };

    let args = Vec::from(args);
    let object_type = match helpers::load_object_type_with(&path) {
        Ok(obj) => obj,
        Err(err) => {
            eprint!("\"{}\":\n{:#?}\n\n", path.bright_white(), err);
            helpers::exit_with_help_string()
        },
    };
    match handler {
        Some(handler) => handler.handle_object(object_type, args),
        None => DefaultHandler::new(Printer {}).handle_object(object_type, args),
    }
}

fn requires_help(args: Vec<String>) -> bool {
    const H_FLAG: &str = "h";
    let mut opts = Options::new();
    opts.optflag(H_FLAG, "help", "");
    match opts.parse(args) {
        Ok(m) => m.opt_present(H_FLAG),
        Err(_) => false,
    }
}

fn matched_handler(name: &str) -> Option<Box<dyn Handler>> {
    for handler in available_handlers() {
        if handler.can_handle_with_name(name) {
            return Some(handler);
        }
    }

    None
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
