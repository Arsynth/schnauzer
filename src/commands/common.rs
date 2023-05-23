use crate::{output::Printer, MachHeader};
use colored::Colorize;

pub(super) fn out_single_arch_title(printer: &Printer, header: &MachHeader, index: usize) {
    let head = format!(
        "{} {}{}",
        "Arch".bold().bright_white(),
        "#".dimmed(),
        index.to_string().bold().bright_white()
    );
    let tail = format!(
        "CPU type: {}, Subtype: {}, File type: {}, Flags: {}",
        header.cputype.to_string().green(),
        header.cpusubtype.masked().to_string().green(),
        header.filetype.to_string().green(),
        format!("{:?}", header.flags).green()
    );
    printer.print_line(format!(
        "{head} {}{tail}{}",
        "(".bright_white(),
        "):".bright_white()
    ));
}

pub(super) fn colored_path_string(path: impl std::fmt::Display) -> String {
    let path = path.to_string();

    let mut parts: Vec<String> = path.split("/").map(|s| s.to_string()).collect();

    let len = parts.len();
    for idx in 0..parts.len() {
        let part = &parts[idx];
        let updated = if idx == len - 1 {
            format!("{}", part.yellow())
        } else if part.starts_with("@") {
            format!("{}", part.trim().blue())
        } else {
            format!("{}", part.trim().green())
        };

        parts[idx] = updated;
    }

    parts.join("/")
}