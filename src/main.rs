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

    pub fn new_list_item() -> Self {
        let tail = format! {"{}{}", " |", "#".dimmed()};
        let body = format!("{}", "    ".dimmed());
        DashLine::new("|", &body, &tail)
    }
}

enum Printable {
    String(String),
    U32(u32),
    /// Value and hex output width
    Hex(u32, usize),
}

impl Debug for Printable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::String(arg0) => write!(f, "{}", arg0.green()),
            Self::U32(arg0) => write!(f, "{}", arg0.to_string().green()),
            Self::Hex(arg0, width) => {
                let arg0 = format!("{:#0w$x}", arg0, w = width);
                write!(f, "{}", arg0.green()) 
            },
        }
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

    out_dashed_field("cputype", Printable::U32(arch.cputype), 1);
    out_dashed_field("cpusubtype", Printable::U32(arch.masked_cpu_subtype()), 1);
    out_dashed_field("offset", Printable::U32(arch.offset), 1);
    out_dashed_field("size", Printable::U32(arch.size), 1);
    out_dashed_field("align", Printable::U32(arch.align), 1);

    handle_macho(arch.object().unwrap(), true);
}

fn handle_macho(macho: MachObject, nested: bool) {
    let level = match nested {
        true => 2,
        false => 1,
    };
    out_header("Mach header:", level);

    let h = macho.header();
    out_dashed_field("magic", Printable::Hex(h.magic.raw_value(), 9), level);
    out_dashed_field("cputype", Printable::U32(h.masked_cpu_subtype()), level);
    out_dashed_field("filetype", Printable::U32(h.file_type()), level);
    out_dashed_field("ncmds", Printable::U32(h.ncmds), level);
    out_dashed_field("flags", Printable::Hex(h.flags, 9), level);

    handle_load_commands(macho.load_commands_iterator(), level + 1);
}

fn handle_load_commands(commands: LoadCommandIterator, level: usize) {
    out_header("Load commands:", level);
    for (index, cmd) in commands.enumerate() {
        out_list_item_dash(level, index);
        out_field("cmd", Printable::String(fmt_ext::load_commang_to_string(cmd.cmd)), " ");
        out_field("cmdsize", Printable::U32(cmd.cmdsize), "\n");
    }
}

fn out_header(hdr: &str, level: usize) {
    print!("{}", DashLine::new_header().get_string(level));
    print!("{}", hdr.bright_white());
    println!("");
}

fn out_dashed_field(name: &str, value: Printable, level: usize) {
    out_field_dash(level);
    out_field(name, value, "\n");
}

fn out_field_dash(level: usize) {
    print!("{}", DashLine::new_field().get_string(level + 1));
}

fn out_list_item_dash(level: usize, index: usize) {
    print!(
        "{}[{}] ",
        DashLine::new_list_item().get_string(level + 1),
        index.to_string().red()
    );
}

use std::io::Write;
fn out_field(name: &str, value: Printable, delimiter: &str) {
    if name.len() > 0 {
        print!("{name}: {:?}", value);
    }
    print!("{delimiter}");
}
