use colored::{self, ColoredString, Colorize};

pub struct Printer {

}

impl Printer {
    pub(crate) fn out_dashed_field(&self, name: String, value: String, level: usize) {
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
            " ".repeat(level + 1),
            index.to_string().red()
        );
    }
    
    pub(crate) fn out_default_colored_field(&self, name: String, value: String, delimiter: &str) {
        self.out_field(name.white(), value.green(), delimiter);
    }
    
    pub(crate) fn out_field(&self, name: ColoredString, value: ColoredString, delimiter: &str) {
        if name.len() > 0 {
            print!("{name}: {value}");
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
}