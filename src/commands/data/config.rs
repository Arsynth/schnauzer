use getopts::Options;

use crate::{commands::common::{options::*, Format}, result::Error};

const COMMON_ERROR: &str = "Unable to obtain section";

const SECT_FLAG_SHORT: &str = "s";
const SECT_FLAG_LONG: &str = "sect";

pub(super) struct Config {
    pub(super) format: Format,
    pub(super) seg: String,
    pub(super) sect: String,
}

impl Config {
    pub(super) fn build(opts: &mut Options, args: &[String]) -> crate::result::Result<Self> {
        let matches = match opts.parse(args.clone()) {
            Ok(m) => m,
            Err(f) => return Err(crate::result::Error::Text(f.to_string())),
        };

        let _ = match matches.opt_positions(SECT_FLAG_SHORT).first() {
            Some(pos) => pos,
            None => return Err(Error::Text(COMMON_ERROR.to_string())),
        };

        match Self::search_after_sect_opt(args) {
            Ok((seg, sect)) => {
                Ok(Self {
                    format: Format::build(opts, args)?,
                    seg: seg.clone(),
                    sect: sect.clone(),
                })
            },
            Err(e) => Err(e),
        }
    }

    fn search_after_sect_opt(args: &[String]) -> crate::result::Result<(&String, &String)> {
        let short = &format!("-{SECT_FLAG_SHORT}");
        let long = &format!("--{SECT_FLAG_LONG}");
        let pos = args.iter().enumerate().find(|a| {
            let arg = a.1;
            arg == short || arg == long
        });

        match pos {
            Some((pos, _)) => Self::search_names_at(args, pos + 1),
            None => Err(Error::Text(COMMON_ERROR.to_string())),
        }
    }

    fn search_names_at(args: &[String], pos: usize) -> crate::result::Result<(&String, &String)> {
        const ERROR_STR: &str = "Incorrect name pattern. Provide \"segname sectname\"";
        let strs = args.get(pos..=pos+1);
        match strs {
            Some(strs) => {
                let filtered: Vec<&String> = strs.iter().filter(|s| !s.starts_with("-")).collect();
                if filtered.len() == 2 {
                    Ok((filtered[0], filtered[1]))
                } else {
                    return Err(Error::Text(ERROR_STR.to_string()));
                }
            },
            None => Err(Error::Text(ERROR_STR.to_string())),
        }
    }
}

impl Config {
    fn required_option_items() -> Vec<OptionItem> {
        vec![
            OptionItem {
                option_type: OptionType::Arg(IsRequired(true)),
                name: OptionName::ShortLong(SECT_FLAG_SHORT.to_string(), SECT_FLAG_LONG.to_string()),
                description: "Section to display".to_string(),
                hint: "segname sectname".to_string(),
            },
        ]
    }

    pub(super) fn option_items() -> Vec<OptionItem> {
        let mut items = Self::required_option_items();
        items.append(&mut Format::option_items());
        items
    }
}
