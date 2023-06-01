use std::fmt::{Display};
use colored::*;

pub struct FixedTabLine<const W: usize> {
    lengts: [usize; W]
}

impl<const W: usize> FixedTabLine<W> {
    pub fn new(lengts: [usize; W]) -> Self {
        Self { lengts }
    }
}

impl<const W: usize> FixedTabLine<W> {
    /// Arguments:
    /// `colors_pattern` - may be empty. If item string index is out of bounds, last color will used
    pub fn print_line(&self, line: [impl Display; W], colors_pattern: Vec<Color>) {
        for (idx, item) in line.iter().enumerate() {
            let item = format!("{item}");
            let spacing = std::cmp::max(self.lengts[idx] as isize - item.len() as isize, 0) as usize;
            
            let colored_item = match colors_pattern.get(idx) {
                Some(color) => item.color(*color).to_string(),
                None => match colors_pattern.last() {
                    Some(color) => item.color(*color).to_string(),
                    None => item,
                }
            };
            
            print!("{}", colored_item + &" ".repeat(spacing));
        }
        println!("");
    }
}