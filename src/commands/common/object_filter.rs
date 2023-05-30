use getopts::*;

use crate::{ObjectType, FatArch, MachObject};
use super::options::*;
use crate::result::{Result, Error};

const ARCH_ARG_SHORT: &str = "a";
const ARCH_ARG_LONG: &str = "arch";

pub(crate) struct ObjectFilter {
    arch: Option<String>,
}

impl ObjectFilter {
    pub(crate) fn build(opts: &mut Options, args: Vec<String>) -> Result<Self> {
        Self::option_item().add_to_opts(opts);

        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(_) => return Err(Error::CantParseArguments),
        };

        Ok(Self {
            arch: matches.opt_str(ARCH_ARG_SHORT),
        })
    }
}

impl ObjectFilter {
    pub(crate) fn get_objects(&self, object_type: ObjectType) -> Vec<MachObject> {
        match &self.arch {
            Some(arch) => match object_type.mach_object_with_arch(&arch) {
                Some(o) => vec![o],
                None => vec![],
            } ,
            None => object_type.mach_objects(),
        }
    }

    pub(crate) fn option_item() -> OptionItem {
        OptionItem {
            option_type: OptionType::Arg(IsRequired(false)),
            name: OptionName::ShortLong(ARCH_ARG_SHORT.to_string(), ARCH_ARG_LONG.to_string()),
            description: format!("Filter architecture by name. Supported archs: x86_64, x86_64h, arm64 and arm64e"),
            hint: "NAME".to_string(),
        }
    }
}
