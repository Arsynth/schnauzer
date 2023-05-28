use colored::Colorize;
use getopts::Options;

pub(super) struct Filter {
    /// Print only identifying fields
    pub(super) short: bool,
    /// Display in
    pub(super) show_indices: bool,
    /// What to print
    pub(super) mode: Mode,
}

pub(super) enum Mode {
    Both,
    SegmentsOnly,
    SectionsOnly,
}

impl Mode {
    fn new(show_segs: bool, show_sects: bool) -> Self {
        if (show_segs && show_sects) || (!show_segs && !show_sects) {
            Self::Both
        } else if show_segs {
            Self::SegmentsOnly
        } else {
            Self::SectionsOnly
        }
    }
}

const SEGS_FLAG: &str = "segs";
const SECTS_FLAG: &str = "sects";
const SHORT_FLAG: &str = "short";
const NO_IDX_FLAG: &str = "noidx";

use super::EXEC_NAME;
use super::common::HELP_FLAG_SHORT;
use super::SUBCOMM_NAME;

impl Filter {
    pub(super) fn build(args: Vec<String>) -> Self {
        let opts = Self::options();

        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(_) => {
                eprintln!("{}", help_string());
                std::process::exit(1);
            }
        };

        if matches.opt_present(HELP_FLAG_SHORT) {
            println!("{}", help_string());
            std::process::exit(0);
        }

        Self {
            short: matches.opt_present(SHORT_FLAG),
            show_indices: !matches.opt_present(NO_IDX_FLAG),
            mode: Mode::new(
                matches.opt_present(SEGS_FLAG),
                matches.opt_present(SECTS_FLAG),
            ),
        }
    }

    pub(super) fn options() -> Options {
        let mut opts = super::default_options();
        opts.optflagopt("", SEGS_FLAG, "", "");
        opts.optflagopt("", SECTS_FLAG, "", "");
        opts.optflagopt("", SHORT_FLAG, "", "");
        opts.optflagopt("", NO_IDX_FLAG, "", "");
        opts
    }
}

pub(super) fn help_string() -> String {
    format!(
        "Usage:\n
        {EXEC_NAME} {} [--{SEGS_FLAG}] [--{SECTS_FLAG}] [--{SHORT_FLAG}] [--{NO_IDX_FLAG}]

        {} - Print only segments
        {} - Print only sections
        {} - Print only values and only identifying fields
        {} - Disable printing indices
        ",
        SUBCOMM_NAME.bright_white(),
        format!("--{}", SEGS_FLAG).bright_white(),
        format!("--{}", SECTS_FLAG).bright_white(),
        format!("--{}", SHORT_FLAG).bright_white(),
        format!("--{}", NO_IDX_FLAG).bright_white(),
    )
}
