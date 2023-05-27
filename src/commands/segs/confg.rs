use getopts::Options;
use crate::result::*;

pub(super) struct Filter {
    archs: bool,
    fields: bool,
    index: bool,
}

pub(super) enum Config {
    SegmentsOnly(Filter),
    SectionsOnly(Filter),
}

impl Config {
    pub(super) fn build(args: Vec<String>) -> Result<Self> {
        // let opts = Options::new();
        //opts.optflagopt("", "segs", desc, hint)
        Ok(Self::SectionsOnly(Filter { archs: true, fields: true, index: true }))
    }
}