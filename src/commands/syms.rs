use super::handler::*;
use super::helpers::args_after_command_name;
use super::helpers::load_object_type_with;
use super::Printer;
use super::Result;
use crate::*;
use colored::*;
use super::common;

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
            ObjectType::MachO(macho) => self.handle_macho(macho, 0),
        }
    }

    fn handle_fat(&self, fat: FatObject) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_arch(arch, index + 1);
            println!("");
        }
    }

    fn handle_arch(&self, arch: FatArch, index: usize) {
        let object = arch.object().unwrap();
        self.handle_macho(object, index);
    }

    fn handle_macho(&self, macho: MachObject, index: usize) {
        common::out_single_arch_title(&self.printer, &macho.header(), index);
        self.handle_load_commands(macho.load_commands_iterator());
    }

    fn handle_load_commands(&self, commands: LoadCommandIterator) {
        let commands = commands.flat_map(|cmd| match cmd.variant {
            LcVariant::Symtab(symtab) => Some(symtab),
            _ => None,
        });
        for cmd in commands {
            self.handle_symtab_command(cmd);
        }
    }

    fn handle_symtab_command(&self, symtab: LcSymtab) {
        for (index, nlist) in symtab.nlist_iterator().enumerate() {
            self.handle_nlist(nlist, index);
        }
    }

    fn handle_nlist(&self, nlist: NlistVariant, index: usize) {
        self.printer.out_list_item_dash(0, index);
        let name = match nlist {
            NlistVariant::Nlist32(nlist) => nlist.name,
            NlistVariant::Nlist64(nlist) => nlist.name,
        };
        self.printer.print_colored_string(name.load_string().unwrap().yellow());
        println!("");
    }
}
