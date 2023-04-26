use std::fs;
use std::path::Path;
use schnauzer::ObjectType;
use schnauzer::Parser;

fn main() {
    let mut args = std::env::args();
    let _exec_name = args.next();

    let path = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Not enough arguments. Provide a valid path to binary");
            std::process::exit(1);
        },
    };
    let path = Path::new(&path);

    let buf = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Could not read file at '{:?}': {e}", path);
            std::process::exit(1);
        },
    }; 
    
    let object = match Parser::parse(&buf) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error while parsing: {:#?}", e);
            std::process::exit(1);
        },
    };

    handle_object(object);
    
}

fn handle_object(obj: ObjectType) {
    println!("***Object***");
    println!("{:#?}", obj);
}