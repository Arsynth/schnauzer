use getopts::*;

use super::common::*;
use super::handler::*;
use super::Printer;
use super::Result;
use crate::auto_enum_fields::Field;
use crate::*;
use super::common::options::*;

static SUBCOMM_NAME: &str = "fat";

pub(super) struct ArchsHandler {
    printer: Printer,
}

impl ArchsHandler {
    pub(super) fn new(printer: Printer) -> Self {
        Self { printer }
    }
}

impl Handler for ArchsHandler {
    fn command_name(&self) -> String {
        SUBCOMM_NAME.to_string()
    }

    fn description(&self) -> String {
        "Prints all fat headers if they exists".to_string()
    }

    fn can_handle_with_name(&self, name: &str) -> bool {
        SUBCOMM_NAME == name
    }

    fn handle_object(&self, object: ObjectType, other_args: Vec<String>) -> Result<()> {
        let mut opts = Options::new();
        self.accepted_option_items().add_to_opts(&mut opts);

        let filter = ObjectFilter::build(&mut opts, &other_args)?;

        let archs = &filter.get_archs(object);
        let out_index = archs.len() > 1;
        for (idx, arch) in archs.iter().enumerate() {
            if out_index {
                self.printer.out_list_item_dash(0, idx);
            }
            self.handle_arch(arch)
        }

        Ok(())
    }
}

const OFFSET_STR: &str = "Offset";
const SIZE_STR: &str = "Size";
const ALIGN_STR: &str = "Align";

impl ArchsHandler {
    fn handle_arch(&self, arch: &FatArch) {
        let mut fields = match arch.printable_cpu() {
            Some(cpu) => {
                vec![Field::new(ARCH_STR.to_string(), cpu.to_string())]
            }
            None => {
                vec![
                    Field::new(CPU_TYPE_STR.to_string(), arch.cputype.to_string()),
                    Field::new(
                        CPU_SUBTYPE_STR.to_string(),
                        arch.cpusubtype.masked().to_string(),
                    ),
                ]
            }
        };

        fields.append(&mut vec![
            Field::new(OFFSET_STR.to_string(), arch.offset.to_string()),
            Field::new(SIZE_STR.to_string(), arch.size.to_string()),
            Field::new(ALIGN_STR.to_string(), arch.align.to_string()),
        ]);

        self.printer.out_default_colored_fields(fields, "\n")
    }
}
