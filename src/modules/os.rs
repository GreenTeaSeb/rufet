use crate::borders::Border;
use crate::color::Rule;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Os {
    format: String,
    border: Border,
    height: usize,
    padding: usize,
    margin: usize,
    alignment: String,
    rule: Vec<Rule>,
}
impl Module for Os {
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
        match sys_info::linux_os_release() {
            Ok(info) => info.pretty_name.unwrap_or_default(),
            Err(_) => String::default(),
        }
    }
}

impl Default for Os {
    fn default() -> Self {
        Self {
            format: String::from("\u{1b}[38;2;255;255;255;49;1mOs:\u{1b}[0m $value"),
            border: Border::default(),
            margin: 0,
            padding: 0,
            height: 0,
            alignment: String::from("left"),
            rule: vec![],
        }
    }
}
