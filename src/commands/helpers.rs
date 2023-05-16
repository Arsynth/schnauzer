use std::env::Args;

use crate::ObjectType;
use crate::Parser;
use std::{path::Path};

/// Function assumes `args` stands on valid path to the file
/// 
/// # Panics
/// Program exits with error if function could not open or parse
/// object or fat object file
pub(crate) fn load_object_type_with(args: &mut Args) -> ObjectType {
    let path = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Not enough arguments. Provide a valid path to binary");
            std::process::exit(1);
        }
    };

    let path = Path::new(&path);

    let parser = match Parser::build(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Could not create parser at '{:?}': {e}", path);
            std::process::exit(1);
        }
    };

    let object = match parser.parse() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error while parsing: {:#?}", e);
            std::process::exit(1);
        }
    };

    object
}

pub(crate) fn args_after_command_name(name: String) -> Option<Args> {
    let mut args = std::env::args();
        let _exec_name = args.next();
        match args.next() {
            Some(subcomm) => match subcomm == name {
                true => Some(args),
                false => None,
            },
            _ => return None,
        }
}