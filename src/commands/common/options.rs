pub(crate) struct OptionItem {
    pub(crate) option_type: OptionType,
    pub(crate) name: OptionName,
    pub(crate) description: String,
    pub(crate) hint: String,
}

use colored::Colorize;
use getopts::*;

pub(crate) trait AddToOptions {
    fn add_to_opts(&self, opts: &mut Options);
}

impl AddToOptions for [OptionItem] {
    fn add_to_opts(&self, opts: &mut Options) {
        for item in self.iter() {
            item.add_to_opts(opts);
        }
    }
}

impl AddToOptions for OptionItem {
    fn add_to_opts(&self, opts: &mut Options) {
        match self.option_type {
            OptionType::Arg(req) => match req.0 {
                true => opts.opt(
                    &self.name.short_or_empty(),
                    &self.name.long_or_empty(),
                    &self.description,
                    &self.hint,
                    HasArg::Yes,
                    Occur::Req,
                ),
                false => opts.opt(
                    &self.name.short_or_empty(),
                    &self.name.long_or_empty(),
                    &self.description,
                    &self.hint,
                    HasArg::Yes,
                    Occur::Optional,
                ),
            },
            OptionType::Flag(req) => match req.0 {
                true => opts.optflag(
                    &self.name.short_or_empty(),
                    &self.name.long_or_empty(),
                    &self.description,
                ),
                false => opts.optflagopt(
                    &self.name.short_or_empty(),
                    &self.name.long_or_empty(),
                    &self.description,
                    &self.hint,
                ),
            },
        };
    }
}

impl OptionItem {
    pub(crate) fn usage_arg_list_item_string(&self) -> String {
        let label = self.label();

        match self.option_type.is_required() {
            true => label.to_string(),
            false => format!("[{}]", label.to_string()),
        }
    }

    pub(crate) fn usage_description_item_string(&self) -> String {
        let label = self.label();
        format!("{label} - {}", self.description)
    }

    fn label(&self) -> String {
        let name = match &self.name {
            OptionName::Long(s) | OptionName::ShortLong(_, s) => format!("--{s}"),
        };

        if self.hint.len() > 0 {
            format!("{} <{}>", name.bright_white(), &self.hint)
        } else {
            name.bright_white().to_string()
        }
    }
}

pub(crate) enum OptionType {
    Arg(IsRequired),
    Flag(IsRequired),
}

impl OptionType {
    fn is_required(&self) -> bool {
        match self {
            OptionType::Arg(r) | OptionType::Flag(r) => r.0,
        }
    }
}

pub(crate) enum OptionName {
    Long(String),
    /// First - short, second - long
    ShortLong(String, String),
}

impl OptionName {
    fn short_or_empty(&self) -> String {
        match self {
            Self::ShortLong(s, _) => s.to_string(),
            _ => "".to_string(),
        }
    }

    fn long_or_empty(&self) -> String {
        match self {
            Self::Long(s) | Self::ShortLong(_, s) => s.to_string(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct IsRequired(pub(crate) bool);
impl Copy for IsRequired {}
