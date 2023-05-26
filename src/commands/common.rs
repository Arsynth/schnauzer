use crate::{output::Printer, MachHeader};
use colored::Colorize;

pub(super) const MAGIC_STR: &str = "Magic";
pub(super) const CPU_TYPE_STR: &str = "CPU type";
pub(super) const CPU_SUBTYPE_STR: &str = "CPU subtype";
pub(super) const ARCH_STR: &str = "Arch";
pub(super) const CAPS_STR: &str = "Capabilities";
pub(super) const FILETYPE_STR: &str = "File type";
pub(super) const N_CMDS_STR: &str = "Commands";
pub(super) const SIZE_OF_CMDS_STR: &str = "Size of commands";
pub(super) const FLAGS_STR: &str = "Flags";

pub(super) fn out_single_arch_title(printer: &Printer, header: &MachHeader, index: usize) {
    let head = format!(
        "{} {}{}",
        ARCH_STR.bold().bright_white(),
        "#".dimmed(),
        index.to_string().bold().bright_white()
    );

    let arch_str = match header.printable_cpu() {
        Some(cpu) => format!("{ARCH_STR}: {}", cpu.to_string().green()),
        None => format!(
            "{CPU_TYPE_STR}: {}, {CPU_SUBTYPE_STR}: {}",
            header.cputype.to_string().green(),
            header.cpusubtype.masked().to_string().green(),
        ),
    };

    let tail = format!(
        "{arch_str}, {FILETYPE_STR}: {}, {FLAGS_STR}: {}",
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
