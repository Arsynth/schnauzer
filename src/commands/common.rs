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
        "CPU type: {}, Subtype: {}, Flags: {}",
        header.cputype.to_string().green(),
        header.masked_cpu_subtype().to_string().green(),
        format!("{:?}", header.flags).green()
    );
    printer.print_line(format!(
        "{head} {}{tail}{}",
        "(".bright_white(),
        "):".bright_white()
    ));
}
