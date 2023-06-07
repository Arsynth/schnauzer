pub mod table;
pub mod hex;

use colored::{self, ColoredString, Colorize};
use std::fmt::Display;

use crate::auto_enum_fields::Field;

pub struct Printer {}

impl Clone for Printer {
    fn clone(&self) -> Self {
        Self {  }
    }
}

impl Printer {
    pub(crate) fn out_dashed_field(&self, name: &str, value: &str, level: usize) {
        self.out_field_dash(level);
        self.out_default_colored_field(name, value, "\n");
    }
    
    pub(crate) fn out_field_dash(&self, level: usize) {
        let tail = format! {"{}{}", "|", "*".dimmed()};
        print!("{}{}", " ".repeat(level + 1), tail);
    }
    
    pub(crate) fn out_list_item_dash(&self, level: usize, index: usize) {
        print!(
            "{}[{}] ",
            " ".repeat(level),
            index.to_string().red()
        );
    }

    pub(crate) fn out_index_path_dash(&self, level: usize, index_path: Vec<usize>) {
        let strings: Vec<String> = index_path.iter().map(|f| f.to_string().red().to_string()).collect();
        let path = strings.join(&":".white().to_string());

        print!(
            "{}[{}] ",
            " ".repeat(level),
            path,
        );
    }
    
    pub(crate) fn out_default_colored_fields(&self, fields: Vec<Field>, tail: &str) {
        let pairs: Vec<String> = fields.iter().map(|f| {
            ColoredField::new_default(&f.name, &f.value).to_string()
        }).collect();

        print!("{}{tail}", pairs.join(", "));
    }

    pub(crate) fn out_default_colored_field(&self, name: &str, value: &str, delimiter: &str) {
        self.out_field(name.white(), value.green(), delimiter);
    }
    
    pub(crate) fn out_field(&self, name: ColoredString, value: ColoredString, delimiter: &str) {
        if name.len() > 0 {
            let field = ColoredField::new(name, value);
            print!("{field}");
        }
        print!("{delimiter}");
    }
    
    pub(crate) fn out_tile(&self, level: usize) {
        self.out_string("-".repeat(20), level)
    }
    
    pub(crate) fn out_string(&self, string: String, level: usize) {
        print!("{}", " ".repeat(level));
        println!("{string}");
    }

    pub(crate) fn print_line(&self, line: impl Display) {
        println!("{line}");
    }

    pub(crate) fn print_string(&self, string: impl Display) {
        print!("{string}");
    }
    
    pub(crate) fn print_strings(&self, strings: Vec<impl Display>, sep: &str) {
        let v: Vec<String> = strings.iter().map(|s| format!("{s}")).collect();
        print!("{}", v.join(sep));
    }

    pub(crate) fn print_colored_string(&self, string: ColoredString) {
        print!("{string}");
    }
}

struct ColoredField {
    string: String
}

impl ColoredField {
    fn new(name: ColoredString, value: ColoredString) -> Self {
        Self {
            string: format!("{}: {}", name, value),
        }
    }

    fn new_default(name: &str, value: &str) -> Self {
        Self {
            string: format!("{}: {}", name.white(), value.green()),
        }
    }
}

impl std::fmt::Display for ColoredField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.string)
    }
}