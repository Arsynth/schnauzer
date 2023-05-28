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
use getopts::Options;
use handler::*;
use headers::*;
use rpaths::*;
use segs::*;
use syms::*;

use common::EXEC_NAME;

use std::process::exit;

pub fn handle_with_args() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", common::HELP_STRING);
        exit(1);
    }

    let handler = &matched_handler(&args[1]);

    if requires_help(handler::default_options(), args.clone()) {
        println!("{}", help_string(handler));
        exit(0);
    }

    let args = match handler {
        Some(handler) => match args.get(2..) {
            Some(args) => args,
            None => {
                eprintln!("{}", handler.help_string());
                exit(1);
            }
        },
        None => match args.get(1..) {
            Some(args) => args,
            None => {
                eprintln!("{}", common::HELP_STRING);
                exit(1);
            }
        },
    };

    if args.len() == 0 {
        eprintln!("{}", common::HELP_STRING);
        exit(1);
    }

    let opts = match handler {
        Some(h) => h.options(),
        None => handler::default_options(),
    };

    let path = match opts.parse(args.clone()) {
        Ok(m) => match m.opt_defined(common::PATH_OPT_SHORT) {
            true => match m.opt_str(common::PATH_OPT_SHORT) {
                Some(path) => path,
                None => args[0].clone(),
            },
            false => args[0].clone(),
        },
        Err(f) => {
            eprintln!("{f}\n\n{}", help_string(handler));
            exit(1);
        }
    };

    let args = Vec::from(args);
    let object_type = match helpers::load_object_type_with(&path) {
        Ok(obj) => obj,
        Err(err) => {
            eprint!("\"{}\":\n{:#?}\n\n", path.bright_white(), err);
            eprintln!("{}", common::HELP_STRING);
            exit(1);
        }
    };
    match handler {
        Some(handler) => handler.handle_object(object_type, args),
        None => DefaultHandler::new(Printer {}).handle_object(object_type, args),
    }
}

fn help_string(handler: &Option<Box<dyn Handler>>) -> String {
    match handler {
        Some(handler) => {
            handler.help_string()
        }
        None => {
            common::HELP_STRING.to_string()
        }
    }
}

fn requires_help(opts: Options, args: Vec<String>) -> bool {
    match opts.parse(args) {
        Ok(m) => m.opt_present(common::HELP_FLAG_SHORT),
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
