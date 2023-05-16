use super::handler::*;
use super::helpers::args_after_command_name;
use super::helpers::load_object_type_with;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::*;
use crate::*;
use colored::*;
pub use std::env::Args;

static SUBCOMM_NAME: &str = "syms";

pub(super) struct SymsHandler {
    pub(super) printer: Printer,
}

impl SymsHandler {
    pub(super) fn new(printer: Printer) -> Self {
        SymsHandler { printer }
    }
}

impl Handler for SymsHandler {
    fn can_handle_with_args(&self) -> bool {
        match args_after_command_name(SUBCOMM_NAME.to_string()) {
            Some(_) => true,
            None => false,
        }
    }

    fn handle_with_args(&self) -> Result<()> {
        match args_after_command_name(SUBCOMM_NAME.to_string()) {
            Some(mut args) => {
                let obj = load_object_type_with(&mut args);
                self.handle_object(obj);
                Ok(())
            }
            None => Err(result::Error::InvalidArgumentsToCmd(
                SUBCOMM_NAME.to_string(),
                std::env::args(),
            )),
        }
    }
}

impl SymsHandler {
    fn handle_object(&self, obj: ObjectType) {
        match obj {
            ObjectType::Fat(fat) => self.handle_fat(fat),
            ObjectType::MachO(macho) => self.handle_macho(macho, false),
        }
    }

    fn handle_fat(&self, fat: FatObject) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1);
        }
    }

    fn handle_arch(&self, arch: FatArch, index: usize) {
        let title = format!(
            "{} {}{}:",
            "Arch".bold().bright_white(),
            "#".dimmed(),
            index.to_string().bold().bright_white()
        );
        println!("{title}");
        self.handle_macho(arch.object().unwrap(), true);
    }

    fn handle_macho(&self, macho: MachObject, nested: bool) {
        let level = match nested {
            true => 1,
            false => 0,
        };

        let h = macho.header();
        for field in h.all_fields() {
            self.printer
                .out_dashed_field(field.name, field.value, level);
        }
        self.printer
            .out_dashed_field("Symbols".to_string(), "".to_string(), level);

        self.handle_load_commands(macho.load_commands_iterator(), level + 1);
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator, level: usize) {
        let commands = commands.flat_map(|cmd| match cmd.variant {
            LcVariant::Symtab(symtab) => Some(symtab),
            _ => None,
        });
        for cmd in commands {
            self.handle_symtab_command(cmd, level);
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
            self.printer
                .out_field(field.name.bright_white(), field.value.yellow(), " ");
        }
        println!("");
    }
}
