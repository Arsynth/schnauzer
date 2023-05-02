use schnauzer::*;
use std::path::Path;
use std::fmt::*;
use colored::{self, Colorize};

fn main() {
    let mut args = std::env::args();
    let _exec_name = args.next();

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

    handle_object(object);
}

fn handle_object(obj: ObjectType) {
    match obj {
        ObjectType::Fat(fat) => handle_fat(fat),
        ObjectType::MachO(macho) => handle_macho(macho, false),
    }
}

fn handle_fat(fat: FatObject) {
    header("Fat:", 0);
    for arch in fat.arch_iterator() {
        handle_arch(arch);
    }
}

fn handle_arch(arch: FatArch) {
    header("Fat arch:", 1);

    field("cputype", &arch.cputype.to_string(), 1);
    field("cpusubtype", &arch.masked_cpu_subtype().to_string(), 1);
    field("offset", &arch.offset.to_string(), 1);
    field("size", &arch.size.to_string(), 1);
    field("align", &arch.align.to_string(), 1);

    handle_macho(arch.object().unwrap(), true);
}

fn handle_macho(macho: MachObject, nested: bool) {
    let level = match nested {
        true => 2,
        false => 1,
    };
    header("Mach header:", level);

    let h = macho.header();
    field("magic", &h.magic.raw_value().hex_string(9), level);
    field("cputype", &h.masked_cpu_subtype().to_string(), level);
    field("filetype", &h.file_type().to_string(), level);
    field("ncmds", &h.ncmds.to_string(), level);
    field("flags", &h.flags.hex_string(9), level);

    header("Load commands:", level);
    for cmd in macho.load_commands_iterator() {
        handle_load_command(cmd, level + 1);
    }
}

fn handle_load_command(cmd: LoadCommand, level: usize) {
    field("cmd", &fmt_ext::load_commang_to_string(cmd.cmd), level);
    field("cmdsize", &cmd.cmdsize.to_string(), level);
}

fn header(hdr: &str, level: usize) {
    if level > 0 {
        print!("|");
    }
    if level > 1 {
        print!("{}", "    |".repeat(level - 1));
    }
    if level > 0 {
        print!("----");
    }
    print!("{hdr}");
    println!("");
}

fn field(name: &str, value: &str, level: usize) {
    field_delimited(name, value, level, "\n");
}

fn field_delimited(name: &str, value: &str, level: usize, delimiter: &str) {
    print!("|");
    if level > 0 {
        print!("{}", "    |".repeat(level));
    }
    if name.len() > 0 || value.len() > 0 {
        print!("{name}: {}", value.yellow());
    }
    print!("{delimiter}");
}

trait ToHex: Display {
    fn hex_string(&self, width: usize) -> String
    where
        Self: LowerHex,
    {
        format!("{:#0w$x}", self, w = width)
    }
}

impl<T> ToHex for T where T: Display {}
