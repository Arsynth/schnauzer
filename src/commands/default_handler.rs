pub use std::env::Args;
use std::{path::Path};
use super::Printer;
use super::Result;
use crate::*;
use crate::auto_enum_fields::*;
use colored::*;

pub(super) struct DefaultHandler {
    printer: Printer,
}

impl DefaultHandler {
    pub(super) fn new(printer: Printer) -> Self {
        DefaultHandler { printer }
    }
}

impl DefaultHandler {
    pub(super) fn handle_with_args(&self) -> Result<()> {
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

    self.handle_object(object);

    Ok(())
    }
}

impl DefaultHandler {
    fn handle_object(&self, obj: ObjectType) {
        match obj {
            ObjectType::Fat(fat) => self.handle_fat(fat),
            ObjectType::MachO(macho) => self.handle_macho(macho, false),
        }
    }
    
    fn handle_fat(&self, fat: FatObject) {
        for arch in fat.arch_iterator() {
            self.handle_arch(arch);
        }
    }
    
    fn handle_arch(&self, arch: FatArch) {
        println!("{}", "Fat arch:".bold().bright_white());
    
        for field in arch.all_fields() {
            self.printer.out_dashed_field(field.name, field.value, 0);
        }
        self.printer.out_dashed_field("Mach header".to_string(), "".to_string(), 0);
    
        self.handle_macho(arch.object().unwrap(), true);
    }
    
    fn handle_macho(&self, macho: MachObject, nested: bool) {
        let level = match nested {
            true => 2,
            false => 1,
        };
    
        let h = macho.header();
        for field in h.all_fields() {
            self.printer.out_dashed_field(field.name, field.value, level);
        }
        self.printer.out_dashed_field("Load commands".to_string(), "".to_string(), level);
    
        self.handle_load_commands(macho.load_commands_iterator(), level + 1);
    }
    
    fn handle_load_commands(&self, commands: LoadCommandIterator, level: usize) {
        for (index, cmd) in commands.enumerate() {
            self.printer.out_list_item_dash(level, index);
            self.printer.out_field(
                "cmd".bright_white(),
                fmt_ext::load_command_to_string(cmd.cmd).yellow(),
                " ",
            );
            self.printer.out_field("cmdsize".bright_white(), cmd.cmdsize.to_string().yellow(), "\n");
    
            self.handle_command_variant(cmd.variant, level + 1);
        }
    }
    
    fn handle_command_variant(&self, variant: LcVariant, level: usize) {
        for field in variant.all_fields() {
            self.printer.out_field_dash(level);
            self.printer.out_default_colored_field(field.name, field.value, "\n");
        }
        match variant {
            LcVariant::Segment32(seg) => self.handle_segment_command32(seg, level),
            LcVariant::Segment64(seg) => self.handle_segment_command64(seg, level),
            LcVariant::Symtab(symtab) => self.handle_symtab_command(symtab, level),
            _ => (),
        }
    }
    
    fn handle_segment_command32(&self, seg: LcSegment32, level: usize) {
        if seg.nsects > 0 {
            self.printer.out_dashed_field("Sections".to_string(), "".to_string(), level);
        }
        for section in seg.sections_iterator() {
            self.handle_section32(section, level + 1);
        }
    }
    
    fn handle_section32(&self, section: Section32, level: usize) {
        for field in section.all_fields() {
            self.printer.out_dashed_field(field.name, field.value, level);
        }
    }
    
    fn handle_segment_command64(&self, seg: LcSegment64, level: usize) {
        if seg.nsects > 0 {
            self.printer.out_dashed_field("Sections".to_string(), "".to_string(), level);
        }
        for section in seg.sections_iterator() {
            self.handle_section64(section, level + 1);
            self.printer.out_tile(level + 1);
        }
    }
    
    fn handle_section64(&self, section: Section64, level: usize) {
        for field in section.all_fields() {
            self.printer.out_dashed_field(field.name, field.value, level + 1);
        }
    }
    
    fn handle_symtab_command(&self, symtab: LcSymtab, level: usize) {
        for (index, nlist) in symtab.nlist_iterator().enumerate() {
            self.handle_nlist(nlist, level, index);
        }
    }
    
    fn handle_nlist(&self, nlist: NlistVariant, level: usize, index: usize) {
        self.printer.out_list_item_dash(level, index);
        for field in nlist.all_fields() {
            self.printer.out_field(
                field.name.bright_white(),
                field.value.yellow(),
                " ",
            );
        }
        println!("");
    }
}