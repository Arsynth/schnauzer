use super::common;
use super::handler::*;
use super::helpers::args_after_command_name;
use super::helpers::load_object_type_with;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::*;
use crate::*;
use colored::*;

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

    fn handle_nlist(&self, nlist: Nlist, index: usize) {
        self.printer.out_list_item_dash(0, index);

        let title = self.type_title(&nlist);
        let name: String = match &nlist.name {
            Some(name) => match name.load_string() {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("{:?}", err);
                    "".to_string()
                }
            },
            None => "".to_string(),
        };

        let name = if name.len() > 0 {
            name.yellow()
        } else {
            "[No name]".dimmed()
        };

        let label = format!("{title} {name}");

        self.printer.print_line(label);

        let other_fields = self.other_symbol_fields(&nlist);
        if other_fields.len() > 0 {
            self.printer.out_default_colored_fields(other_fields, "\n");
        }
    }

    fn type_title(&self, nlist: &Nlist) -> ColoredString {
        let ntype = &nlist.n_type;
        if let Some(stab) = ntype.stab_type() {
            format!("Stab({:?})", stab)
        } else if ntype.is_private_external() {
            "Private".to_string()
        } else if ntype.is_external() {
            "External".to_string()
        } else if ntype.is_undefined() {
            "Undefined".to_string()
        } else if ntype.is_absolute() {
            "Absolute".to_string()
        } else if ntype.is_defined_in_n_sect() {
            "Symbol".to_string()
        } else if ntype.is_prebound() {
            "Prebound".to_string()
        } else if ntype.is_indirect() {
            "Same as".to_string()
        } else {
            "".to_string()
        }
        .blue()
    }

    fn other_symbol_fields(&self, nlist: &Nlist) -> Vec<Field> {
        self.fields_with_options(nlist, nlist.n_type.options())
    }

    fn fields_with_options(&self, nlist: &Nlist, opts: SymbolOptions) -> Vec<Field> {
        const SECTION_TITLE: &str = "Section";
        const N_SECT_TITLE: &str = "n_sect";
        const N_DESC_TITLE: &str = "n_desc";
        const N_VALUE_TITLE: &str = "n_value";

        let mut fields = vec![];

        let n_sect: Option<Field> = match opts.n_sect {
            NsectOption::None | NsectOption::Unknown => None,
            NsectOption::Some => Some(Field::new(SECTION_TITLE.to_string(), nlist.n_sect.to_string())),
            NsectOption::Zero => Some(Field::new(SECTION_TITLE.to_string(), 0.to_string())),
            NsectOption::Raw => Some(Field::new(N_SECT_TITLE.to_string(), nlist.n_sect.to_string())),
        };
        if let Some(n_sect) = n_sect {
            fields.push(n_sect);
        }

        let n_desc: Option<Field> = match opts.n_desc {
            NdescOption::None | NdescOption::Unknown => None,
            NdescOption::GlobalSymbolType
            | NdescOption::StaticSymbolType
            | NdescOption::LocalCommonSymbolType
            | NdescOption::RegisterType
            | NdescOption::StructureEltType
            | NdescOption::SymbolType
            | NdescOption::ParameterType => {
                Some(Field::new("Type".to_string(), nlist.n_desc.to_string()))
            }
            NdescOption::LineNumber => {
                Some(Field::new("Line".to_string(), nlist.n_desc.to_string()))
            }
            NdescOption::NestingLevel => {
                Some(Field::new("Nesting".to_string(), nlist.n_desc.to_string()))
            }
            NdescOption::Raw => {
                Some(Field::new(N_DESC_TITLE.to_string(), nlist.n_desc.to_string()))
            },
        };
        if let Some(n_desc) = n_desc {
            fields.push(n_desc);
        }

        let n_value: Option<Field> = match opts.n_value {
            NvalueOption::None | NvalueOption::Unknown => None,
            NvalueOption::Address => Some(Field::new(
                "Address".to_string(),
                nlist.n_value.hex_string(),
            )),
            NvalueOption::Register => Some(Field::new(
                "Register".to_string(),
                nlist.n_value.to_string(),
            )),
            NvalueOption::StructOffset | NvalueOption::Offset => {
                Some(Field::new("Offset".to_string(), nlist.n_value.to_string()))
            }
            NvalueOption::LastModTime => Some(Field::new(
                "Last mod".to_string(),
                nlist.n_value.to_string(),
            )),
            NvalueOption::Sum => Some(Field::new("Sum".to_string(), nlist.n_value.to_string())),
            NvalueOption::Length => {
                Some(Field::new("Length".to_string(), nlist.n_value.to_string()))
            }
            NvalueOption::Raw => {
                Some(Field::new(N_VALUE_TITLE.to_string(), nlist.n_value.to_string()))
            },
        };
        if let Some(n_value) = n_value {
            fields.push(n_value)
        }

        fields
    }
}
