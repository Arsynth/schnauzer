use crate::ObjectType;
use crate::Parser;
use super::Result;
use std::{path::Path};

const HELP_STRING: &str = "Help:
# Prints almost all binary info
schnauzer path_to_binary

# Prints symtab
schnauzer syms path_to_binary

# Prints relative paths
schnauzer rpaths path_to_binary

# Prints used dynamic libraries
schnauzer dylibs path_to_binary

# Prints all the segments with sections
schnauzer segs path_to_binary

# Prints the fat archs
schnauzer fat path_to_binary

# Prints headers
schnauzer headers path_to_binary
";

pub(crate) fn load_object_type_with(path: &str) -> Result<ObjectType> {
    let path = Path::new(&path);
    let parser = Parser::build(path)?;
    let object = parser.parse()?;

    Ok(object)
}

pub(crate) fn exit_with_help_string() -> ! {
    eprintln!("{HELP_STRING}");
    std::process::exit(1)
}

pub(crate) fn exit_normally_with_help_string() -> ! {
    println!("{HELP_STRING}");
    std::process::exit(0)
}