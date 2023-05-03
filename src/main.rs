use colored::{self, Colorize};
use schnauzer::*;
use std::fmt::*;
use std::path::Path;

struct DashLine {
    head: String,
    body: String,
    tail: String,
}

impl DashLine {
    pub fn new(head: &str, body: &str, tail: &str) -> Self {
        DashLine {
            head: head.to_string(),
            body: body.to_string(),
            tail: tail.to_string(),
        }
    }

    pub fn new_header() -> Self {
        let body = format!("{}", "----|");
        DashLine::new("|", &body, "----")
    }

    pub fn new_field() -> Self {
        let tail = format! {"{}{}", "|", "*".dimmed()};
        let body = format!("{}", "    ".dimmed());
        DashLine::new("|", &body, &tail)
    }
}

impl DashLine {
    pub fn get_string(&self, size: usize) -> String {
        if size == 0 {
            return "".to_string();
        }

        format!("{}{}{}", self.head, self.body.repeat(size - 1), self.tail)
    }
}

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
    out_header("Fat:", 0);
    for arch in fat.arch_iterator() {
        handle_arch(arch);
    }
}

fn handle_arch(arch: FatArch) {
    out_header("Fat arch:", 1);

    out_field("cputype", &arch.cputype.to_string(), 1);
    out_field("cpusubtype", &arch.masked_cpu_subtype().to_string(), 1);
    out_field("offset", &arch.offset.to_string(), 1);
    out_field("size", &arch.size.to_string(), 1);
    out_field("align", &arch.align.to_string(), 1);

    handle_macho(arch.object().unwrap(), true);
}

fn handle_macho(macho: MachObject, nested: bool) {
    let level = match nested {
        true => 2,
        false => 1,
    };
    out_header("Mach header:", level);

    let h = macho.header();
    out_field("magic", &h.magic.raw_value().hex_string(9), level);
    out_field("cputype", &h.masked_cpu_subtype().to_string(), level);
    out_field("filetype", &h.file_type().to_string(), level);
    out_field("ncmds", &h.ncmds.to_string(), level);
    out_field("flags", &h.flags.hex_string(9), level);

    out_header("Load commands:", level + 1);
    for cmd in macho.load_commands_iterator() {
        handle_load_command(cmd, level + 1);
    }
}

fn handle_load_command(cmd: LoadCommand, level: usize) {
    out_field("cmd", &fmt_ext::load_commang_to_string(cmd.cmd), level);
    out_field("cmdsize", &cmd.cmdsize.to_string(), level);
}

fn out_header(hdr: &str, level: usize) {
    print!("{}", DashLine::new_header().get_string(level));
    print!("{hdr}");
    println!("");
}

fn out_field(name: &str, value: &str, level: usize) {
    out_field_delimited(name, value, level, "\n");
}

fn out_field_delimited(name: &str, value: &str, level: usize, delimiter: &str) {
    print!("{}", DashLine::new_field().get_string(level + 1));

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
