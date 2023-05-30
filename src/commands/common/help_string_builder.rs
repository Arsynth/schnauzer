use colored::Colorize;

use super::options::*;
use super::EXEC_NAME;

pub(crate) struct HelpStringBuilder {
    command: String,
    pub(crate) description: Option<String>,
    pub(crate) show_usage_title: bool,
    items: Vec<OptionItem>,
}

impl HelpStringBuilder {
    pub(crate) fn new(command: String) -> Self {
        Self {
            command,
            description: None,
            show_usage_title: false,
            items: Vec::new(),
        }
    }
}

impl HelpStringBuilder {
    pub(crate) fn add_option_items(&mut self, items: &mut Vec<OptionItem>) {
        self.items.append(items);
    }

    pub(crate) fn build_string(&self) -> String {
        let description = match &self.description {
            Some(description) => format!("\nDescription:\n{} - {}\n", self.command.bright_green(), description.bright_white()),
            None => "".into(),
        };

        let usage = match &self.show_usage_title {
            true => format!("\nUsage:\n"),
            false => "".into(),
        };
        let mut result = format! {"{description}{usage}{EXEC_NAME} {} {} ", self.command.bright_green(), "FILE".bright_white()};

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
