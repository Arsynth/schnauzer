use auto_enum_fields::*;
use colored::{self, ColoredString, Colorize};
use schnauzer::*;
use std::{path::Path};

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
    for arch in fat.arch_iterator() {
        handle_arch(arch);
    }
}

fn handle_arch(arch: FatArch) {
    println!("{}", "Fat arch:".bold().bright_white());

    for field in arch.all_fields() {
        out_dashed_field(field.name, field.value, 0);
    }
    out_dashed_field("Mach header".to_string(), "".to_string(), 0);

    handle_macho(arch.object().unwrap(), true);
}

fn handle_macho(macho: MachObject, nested: bool) {
    let level = match nested {
        true => 2,
        false => 1,
    };

    let h = macho.header();
    for field in h.all_fields() {
        out_dashed_field(field.name, field.value, level);
    }
    out_dashed_field("Load commands".to_string(), "".to_string(), level);

    handle_load_commands(macho.load_commands_iterator(), level + 1);
}

fn handle_load_commands(commands: LoadCommandIterator, level: usize) {
    for (index, cmd) in commands.enumerate() {
        out_list_item_dash(level, index);
        out_field(
            "cmd".bright_white(),
            fmt_ext::load_command_to_string(cmd.cmd).yellow(),
            " ",
        );
        out_field("cmdsize".bright_white(), cmd.cmdsize.to_string().yellow(), "\n");

        handle_command_variant(cmd.variant, level + 1);
    }
}

fn handle_command_variant(variant: LcVariant, level: usize) {
    for field in variant.all_fields() {
        out_field_dash(level);
        out_default_colored_field(field.name, field.value, "\n");
    }
    match variant {
        LcVariant::Segment32(seg) => handle_segment_command32(seg, level),
        LcVariant::Segment64(seg) => handle_segment_command64(seg, level),
        LcVariant::Symtab(symtab) => handle_symtab_command(symtab, level),
        _ => (),
    }
}

fn handle_segment_command32(seg: LcSegment32, level: usize) {
    if seg.nsects > 0 {
        out_dashed_field("Sections".to_string(), "".to_string(), level);
    }
    for section in seg.sections_iterator() {
        handle_section32(section, level + 1);
    }
}

fn handle_section32(section: Section32, level: usize) {
    for field in section.all_fields() {
        out_dashed_field(field.name, field.value, level);
    }
}

fn handle_segment_command64(seg: LcSegment64, level: usize) {
    if seg.nsects > 0 {
        out_dashed_field("Sections".to_string(), "".to_string(), level);
    }
    for section in seg.sections_iterator() {
        handle_section64(section, level + 1);
        out_tile(level + 1);
    }
}

fn handle_section64(section: Section64, level: usize) {
    for field in section.all_fields() {
        out_dashed_field(field.name, field.value, level + 1);
    }
}

fn handle_symtab_command(symtab: LcSymtab, level: usize) {
    for (index, nlist) in symtab.nlist_iterator().enumerate() {
        handle_nlist(nlist, level, index);
    }
}

fn handle_nlist(nlist: NlistVariant, level: usize, index: usize) {
    out_list_item_dash(level, index);
    for field in nlist.all_fields() {
        out_field(
            field.name.bright_white(),
            field.value.yellow(),
            " ",
        );
    }
    println!("");
}

fn out_dashed_field(name: String, value: String, level: usize) {
    out_field_dash(level);
    out_default_colored_field(name, value, "\n");
}

fn out_field_dash(level: usize) {
    let tail = format! {"{}{}", "|", "*".dimmed()};
    print!("{}{}", " ".repeat(level + 1), tail);
}

fn out_list_item_dash(level: usize, index: usize) {
    print!(
        "{}[{}] ",
        " ".repeat(level + 1),
        index.to_string().red()
    );
}

fn out_default_colored_field(name: String, value: String, delimiter: &str) {
    out_field(name.white(), value.green(), delimiter);
}

fn out_field(name: ColoredString, value: ColoredString, delimiter: &str) {
    if name.len() > 0 {
        print!("{name}: {value}");
    }
    print!("{delimiter}");
}

fn out_tile(level: usize) {
    out_string("-".repeat(20), level)
}

fn out_string(string: String, level: usize) {
    print!("{}", " ".repeat(level));
    println!("{string}");
}