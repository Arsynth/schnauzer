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
use getopts::{Options, HasArg, Occur};
use handler::*;
use syms::*;
use rpaths::*;
use dylibs::*;
use segs::*;
use fat::*;
use headers::*;

const HELP_STRING: &str = "Help:
# Prints almost all binary info
schnauzer -p path_to_binary

# Prints symtab
schnauzer syms -p path_to_binary

# Prints relative paths
schnauzer rpaths -p path_to_binary

# Prints used dynamic libraries
schnauzer dylibs -p path_to_binary

# Prints all the segments with sections
schnauzer segs -p path_to_binary

# Prints the fat archs
schnauzer fat -p path_to_binary

# Prints headers
schnauzer headers -p path_to_binary
";

pub fn handle_with_args() -> Result<()> {
    const PATH_OPT: &str = "p";

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        helpers::exit_with_help_string(HELP_STRING);
    } else if requires_help(args.clone()) {
        helpers::exit_normally_with_help_string(HELP_STRING);
    }

    let mut opts = Options::new();
    opts.opt(PATH_OPT, "path", "Path to file", "FILE", HasArg::Yes, Occur::Req);

    let path = match opts.parse(args.clone()) {
        Ok(m) => m.opt_str(PATH_OPT),
        Err(f) => helpers::exit_with_help_string(&f.to_string()),
    };

    if let Some(path) = path {
        let object_type = helpers::load_object_type_with(&path)?;
        match matched_handler(&args[1]) {
            Some(h) => h.handle_object(object_type, args)?,
            None => DefaultHandler::new(Printer {}).handle_object(object_type, args)?,
        };
    } else {
        helpers::exit_with_help_string("Not enough arguments. Provide a valid path to binary");
    }

    Ok(())
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
