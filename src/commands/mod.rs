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

use self::common::help_string_builder::HelpStringBuilder;
use self::common::options::{AddToOptions, OptionItem};

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

use std::process::exit;

pub fn handle_with_args() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", help_string(None));
        exit(1);
    }

    let handler = matched_handler(&args[1]);
    let (command_name, mut option_items) = match &handler {
        Some(h) => (Some(h.command_name()), h.accepted_option_items()),
        None => (None, handler::default_option_items()),
    };

    let mut opts = Options::new();
    option_items.add_to_opts(&mut opts);

    let help_request = match command_name {
        Some(command_name) => Some(HelpStringRequest(command_name.clone(), &mut option_items)),
        None => None,
    };

    if requires_help(handler::default_options(), args.clone()) {
        println!("{}", help_string(help_request));
        exit(0);
    }

    let args = match &handler {
        Some(_) => match args.get(2..) {
            Some(args) => args,
            None => {
                eprintln!("{}", help_string(help_request));
                exit(1);
            }
        },
        None => match args.get(1..) {
            Some(args) => args,
            None => {
                eprintln!("{}", help_string(None));
                exit(1);
            }
        },
    };

    if args.len() == 0 {
        eprintln!("{}", help_string(None));
        exit(1);
    }

    let path = match opts.parse(args.clone()) {
        Ok(m) => match m.opt_defined(common::PATH_OPT_SHORT) {
            true => match m.opt_str(common::PATH_OPT_SHORT) {
                Some(path) => path,
                None => args[0].clone(),
            },
            false => args[0].clone(),
        },
        Err(f) => {
            eprintln!("{f}\n\n{}", help_string(help_request));
            exit(1);
        }
    };

    let args = Vec::from(args);
    let object_type = match helpers::load_object_type_with(&path) {
        Ok(obj) => obj,
        Err(err) => {
            eprint!("\"{}\":\n{:#?}\n\n", path.bright_white(), err);
            exit(1);
        }
    };
    match handler {
        Some(handler) => handler.handle_object(object_type, args),
        None => DefaultHandler::new(Printer {}).handle_object(object_type, args),
    }
}

struct HelpStringRequest<'a>(String, &'a mut Vec<OptionItem>);

fn help_string(request: Option<HelpStringRequest>) -> String {
    match request {
        Some(request) => {
            let mut help_string_builder =
                HelpStringBuilder::new(request.0.clone(), Some("Usage".to_string()));
            help_string_builder.add_option_items(request.1);
            help_string_builder.build_string()
        }
        None => {
            let mut result = "".to_string();
            for handler in available_handlers() {
                let mut help_string_builder = HelpStringBuilder::new(handler.command_name(), None);
                help_string_builder.add_option_items(&mut handler.accepted_option_items());
                result += &help_string_builder.build_string();
                result += "\n";
            }

            result
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
