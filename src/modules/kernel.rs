use crate::borders::Border;
use crate::color::Rule;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Kernel {
    format: String,
    border: Border,
    height: usize,
    padding: usize,
    margin: usize,
    alignment: String,
    rule: Vec<Rule>,
}
impl Module for Kernel {
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
        sys_info::os_release().unwrap_or_default()
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self {
            format: String::from("\u{1b}[38;2;255;255;255;49;1mKernel:\u{1b}[0m $value"),
            border: Border::default(),
            margin: 0,
            padding: 0,
            height: 0,
            alignment: String::from("left"),
            rule: vec![],
        }
    }
}
