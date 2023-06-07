use super::options::*;
use getopts::*;

const SHORT_OUTPUT_FLAG: &str = "short";
const NO_IDX_FLAG: &str = "noidx";

pub(crate) struct Format {
    /// Print only identifying fields
    pub(crate) short: bool,
    /// Display in
    pub(crate) show_indices: bool,
}

impl Format {
    pub(crate) fn build(opts: &mut Options, args: &[String]) -> crate::result::Result<Self> {
        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(f) => return Err(crate::result::Error::Text(f.to_string())),
        };

        Ok(Self {
            short: matches.opt_present(SHORT_OUTPUT_FLAG),
            show_indices: !matches.opt_present(NO_IDX_FLAG),
        })
    }

    pub(crate) fn option_items() -> Vec<OptionItem> {
        vec![
            OptionItem {
                option_type: OptionType::Flag(IsRequired(false)),
                name: OptionName::Long(SHORT_OUTPUT_FLAG.to_string()),
                description: "Display only identifying fields".to_string(),
                hint: "".to_string(),
            },
            OptionItem {
                option_type: OptionType::Flag(IsRequired(false)),
                name: OptionName::Long(NO_IDX_FLAG.to_string()),
                description: "Don't display indices of items".to_string(),
                hint: "".to_string(),
            },
        ]
    }
}
