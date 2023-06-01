use getopts::Options;

use crate::commands::common::{options::*, Format};

const SEGS_FLAG: &str = "segs";
const SECTS_FLAG: &str = "sects";

pub(super) struct Config {
    pub(super) format: Format,
    pub(super) show_segs: bool,
    pub(super) show_sects: bool,
}

impl Config {
    pub(super) fn build(opts: &mut Options, args: &[String]) -> crate::result::Result<Self> {
        Self::required_option_items().add_to_opts(opts);

        let matches = match opts.parse(args.clone()) {
            Ok(m) => m,
            Err(_) => return Err(crate::result::Error::CantParseArguments),
        };

        let segs_only = matches.opt_present(SEGS_FLAG);
        let sects_only = matches.opt_present(SECTS_FLAG);
        let nothing = !(segs_only || sects_only);

        Ok(Self {
            format: Format::build(opts, args)?,
            show_segs: segs_only || nothing,
            show_sects: sects_only || nothing,
        })
    }
}

impl Config {
    fn required_option_items() -> Vec<OptionItem> {
        vec![
            OptionItem {
                option_type: OptionType::Flag(IsRequired(false)),
                name: OptionName::Long(SEGS_FLAG.to_string()),
                description: "Display only segments".to_string(),
                hint: "".to_string(),
            },
            OptionItem {
                option_type: OptionType::Flag(IsRequired(false)),
                name: OptionName::Long(SECTS_FLAG.to_string()),
                description: "Display only sections".to_string(),
                hint: "".to_string(),
            },
        ]
    }

    pub(super) fn option_items() -> Vec<OptionItem> {
        let mut items = Self::required_option_items();
        items.append(&mut Format::option_items());
        items
    }
}
