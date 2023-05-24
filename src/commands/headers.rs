use colored::Colorize;

use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::Field;
use crate::*;

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
    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, _other_args: Vec<String>) -> Result<()> {
        self.handle_object(object);
        Ok(())
    }
}

const MAGIC_STR: &str = "Magic";
const CPU_TYPE_STR: &str = "CPU type";
const CPU_SUBTYPE_STR: &str = "CPU subtype";
const CAPS_STR: &str = "Capabilities";
const FILETYPE_STR: &str = "File type";
const N_CMDS_STR: &str = "Commands";
const SIZE_OF_CMDS_STR: &str = "Size of commands";
const FLAGS_STR: &str = "Flags";

impl HeadersHandler {
    fn handle_object(&self, obj: ObjectType) {
        match obj {
            ObjectType::Fat(fat) => self.handle_fat(fat),
            ObjectType::MachO(macho) => self.handle_mach_header(macho.header(), 0),
        }
    }

    fn handle_fat(&self, fat: FatObject) {
        for (index, arch) in fat.arch_iterator().enumerate() {
            self.handle_mach_header(arch.object().unwrap().header(), index)
        }
    }

    fn handle_mach_header(&self, header: &MachHeader, index: usize) {
        self.printer.out_list_item_dash(0, index);

        let fields = vec![
            Field::new(MAGIC_STR.to_string(), header.magic.to_string()),
            Field::new(CPU_TYPE_STR.to_string(), header.cputype.to_string()),
            Field::new(
                CPU_SUBTYPE_STR.to_string(),
                header.cpusubtype.masked().to_string(),
            ),
            Field::new(
                CAPS_STR.to_string(),
                header.cpusubtype.feature_flags().to_string(),
            ),
            Field::new(FILETYPE_STR.to_string(), header.filetype.to_string()),
            Field::new(N_CMDS_STR.to_string(), header.ncmds.to_string()),
            Field::new(SIZE_OF_CMDS_STR.to_string(), header.sizeofcmds.to_string()),
            Field::new(FLAGS_STR.to_string(), header.flags.to_string()),
        ];
        self.printer.out_default_colored_fields(fields, "\n");

        self.printer.print_line(format!("{}", "Flags(detailed):".bright_white()));
        self.printer.print_line(printable_flags::strings_for_flags(header.flags.0).join("\n"))
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
