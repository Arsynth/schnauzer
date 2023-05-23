use super::handler::*;
use super::Printer;
use super::Result;
use crate::*;
use crate::auto_enum_fields::Field;

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
                Field::new(CPU_SUBTYPE_STR.to_string(), header.cpusubtype.masked().to_string()),
                Field::new(CAPS_STR.to_string(), header.cpusubtype.feature_flags().to_string()),
                Field::new(FILETYPE_STR.to_string(), header.filetype.to_string()),
                Field::new(N_CMDS_STR.to_string(), header.ncmds.to_string()),
                Field::new(SIZE_OF_CMDS_STR.to_string(), header.sizeofcmds.to_string()),
                Field::new(FLAGS_STR.to_string(), header.flags.to_string()),
                
            ];
            self.printer.out_default_colored_fields(fields, "\n")
    }
}
