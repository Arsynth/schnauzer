use colored::Colorize;

use super::options::*;
use super::EXEC_NAME;

pub(crate) struct HelpStringBuilder {
    command: String,
    items: Vec<OptionItem>,
}

impl HelpStringBuilder {
    pub(crate) fn new(command: String) -> Self {
        Self {
            command,
            items: Vec::new(),
        }
    }
}

impl HelpStringBuilder {
    pub(crate) fn add_option_items(&mut self, items: &mut Vec<OptionItem>) {
        self.items.append(items);
    }

    pub(crate) fn build_string(&self) -> String {
        let mut result = format! {"Usage:\n{EXEC_NAME} {} {} ", self.command, "path_to_binary".bright_white()};

        let arg_list: Vec<String> = self
            .items
            .iter()
            .map(|i| i.usage_arg_list_item_string())
            .collect();
        result += &arg_list.join(" ");
        result += "\n\n";

        for desc in self.items.iter().map(|i| i.usage_description_item_string()) {
            result += &format!("\t{desc}\n");
        }

        result
    }
}
