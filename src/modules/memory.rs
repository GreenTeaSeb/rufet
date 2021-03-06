use crate::borders::Border;
use crate::color::Rule;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Memory {
    format: String,
    border: Border,
    height: usize,
    padding: usize,
    margin: usize,
    alignment: String,
    rule: Vec<Rule>,
}
impl Module for Memory {
    fn format(&self) -> String {
        let formated = if !self.rule.is_empty() {
            self.rule.iter().fold(self.format.clone(), |acc, rule| {
                acc.replace(&rule.id, &rule.get_colored())
            })
        } else {
            self.format.clone()
        }
        .replace("$value", &self.get_val())
        .add_margin(&self.padding)
        .align(&self.alignment);

        self.border.add_border(&formated).add_margin(&self.margin)
    }
    fn get_val(&self) -> String {
        match sys_info::mem_info() {
            Ok(mem) => format!(
                "{:.4}",
                ((mem.total - mem.free - (mem.buffers + mem.cached)) as f32 / (1024_f32.powf(2.0)))
                    .to_string()
            ),
            _ => String::default(),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            format: String::from("\u{1b}[38;2;255;255;255;49;1mMemory:\u{1b}[0m $value GB"),

            border: Border::default(),

            margin: 0,
            padding: 0,
            height: 0,
            alignment: "left".to_string(),
            rule: vec![],
        }
    }
}
