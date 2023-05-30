use colored::Colorize;

use super::common::options::*;
use super::common::*;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::Field;
use crate::*;
use getopts::*;

static SUBCOMM_NAME: &str = "headers";

pub(super) struct HeadersHandler {
    printer: Printer,
}

impl HeadersHandler {
    pub(super) fn new(printer: Printer) -> Self {
        Self { printer }
    }
}

impl Handler for HeadersHandler {
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let mut opts = Options::new();
        self.accepted_option_items().add_to_opts(&mut opts);

        let format = &Format::build(&mut opts, &other_args)?;
        let filter = ObjectFilter::build(&mut opts, &other_args)?;

        let objects = &filter.get_objects(object);

        for (idx, obj) in objects.iter().enumerate() {
            self.handle_mach_header(obj.header(), idx, format);
        }

        Ok(())
    }

    fn accepted_option_items(&self) -> Vec<OptionItem> {
        let mut result = default_option_items();
        result.append(&mut Format::option_items());
        result
    }
}

impl HeadersHandler {
    fn handle_mach_header(&self, header: &MachHeader, index: usize, format: &Format) {
        if format.show_indices {
            self.printer.out_list_item_dash(0, index);
        }

        if format.short {
            let mut strings: Vec<String> = vec![header.magic.to_string().green().to_string()];

            let mut cpu_tokens = match header.printable_cpu() {
                Some(cpu) => vec![cpu.to_string().green().to_string()],
                None => vec![
                    header.cputype.to_string().green().to_string(),
                    header.cpusubtype.masked().to_string().green().to_string(),
                ],
            };

            strings.append(&mut cpu_tokens);

            self.printer.print_strings(strings, " ");
            self.printer.print_line("");
        } else {
            let mut fields = vec![Field::new(MAGIC_STR.to_string(), header.magic.to_string())];

            match header.printable_cpu() {
                Some(cpu) => {
                    fields.push(Field::new(ARCH_STR.to_string(), cpu.to_string()));
                }
                None => {
                    fields.append(&mut vec![
                        Field::new(CPU_TYPE_STR.to_string(), header.cputype.to_string()),
                        Field::new(
                            CPU_SUBTYPE_STR.to_string(),
                            header.cpusubtype.masked().to_string(),
                        ),
                    ]);
                }
            };

            fields.append(&mut vec![
                Field::new(
                    CAPS_STR.to_string(),
                    header.cpusubtype.feature_flags().to_string(),
                ),
                Field::new(FILETYPE_STR.to_string(), header.filetype.to_string()),
                Field::new(N_CMDS_STR.to_string(), header.ncmds.to_string()),
                Field::new(SIZE_OF_CMDS_STR.to_string(), header.sizeofcmds.to_string()),
                Field::new(FLAGS_STR.to_string(), header.flags.to_string()),
            ]);

            self.printer.out_default_colored_fields(fields, "\n");

            self.printer
                .print_line(format!("{}", "Flags(detailed):".bright_white()));
            self.printer
                .print_line(printable_flags::strings_for_flags(header.flags.0).join("\n"))
        }
    }
}

mod printable_flags {
    use colored::Colorize;

    use crate::primitives::object_flags_constants::*;

    const ALL_FLAGS: [(u32, &str); 26] = [
        (MH_NOUNDEFS, "MH_NOUNDEFS"),
        (MH_INCRLINK, "MH_INCRLINK"),
        (MH_DYLDLINK, "MH_DYLDLINK"),
        (MH_BINDATLOAD, "MH_BINDATLOAD"),
        (MH_PREBOUND, "MH_PREBOUND"),
        (MH_SPLIT_SEGS, "MH_SPLIT_SEGS"),
        (MH_LAZY_INIT, "MH_LAZY_INIT"),
        (MH_TWOLEVEL, "MH_TWOLEVEL"),
        (MH_FORCE_FLAT, "MH_FORCE_FLAT"),
        (MH_NOMULTIDEFS, "MH_NOMULTIDEFS"),
        (MH_NOFIXPREBINDING, "MH_NOFIXPREBINDING"),
        (MH_PREBINDABLE, "MH_PREBINDABLE"),
        (MH_ALLMODSBOUND, "MH_ALLMODSBOUND"),
        (MH_SUBSECTIONS_VIA_SYMBOLS, "MH_SUBSECTIONS_VIA_SYMBOLS"),
        (MH_CANONICAL, "MH_CANONICAL"),
        (MH_WEAK_DEFINES, "MH_WEAK_DEFINES"),
        (MH_BINDS_TO_WEAK, "MH_BINDS_TO_WEAK"),
        (MH_ALLOW_STACK_EXECUTION, "MH_ALLOW_STACK_EXECUTION"),
        (MH_ROOT_SAFE, "MH_ROOT_SAFE"),
        (MH_SETUID_SAFE, "MH_SETUID_SAFE"),
        (MH_NO_REEXPORTED_DYLIBS, "MH_NO_REEXPORTED_DYLIBS"),
        (MH_PIE, "MH_PIE"),
        (MH_DEAD_STRIPPABLE_DYLIB, "MH_DEAD_STRIPPABLE_DYLIB"),
        (MH_HAS_TLV_DESCRIPTORS, "MH_HAS_TLV_DESCRIPTORS"),
        (MH_NO_HEAP_EXECUTION, "MH_NO_HEAP_EXECUTION"),
        (MH_APP_EXTENSION_SAFE, "MH_APP_EXTENSION_SAFE"),
    ];

    pub(super) fn strings_for_flags(flags: u32) -> Vec<String> {
        let mut result = vec![];
        for tup in ALL_FLAGS {
            if flags & tup.0 > 0 {
                result.push(format!("{}", tup.1.yellow()));
            }
        }
        result
    }
}
