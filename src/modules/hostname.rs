use crate::borders::Border;
use crate::color::Rule;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Hostname {
    format: String,
    border: Border,
    height: usize,
    padding: usize,
    margin: usize,
    alignment: String,
    rule: Vec<Rule>,
}
impl Module for Hostname {
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
        .align("left");

        self.border.add_border(&formated).add_margin(&self.margin)
    }
    fn get_val(&self) -> String {
        sys_info::hostname().unwrap_or_default()
    }
}
impl Default for Hostname {
    fn default() -> Self {
        Self {
            border: Border::default(),
            format: String::from("\u{1b}[38;2;255;255;255;49;1mHostname:\u{1b}[0m $value"),
            margin: 0,
            padding: 0,
            height: 0,
            alignment: String::from("left"),
            rule: vec![],
        }
    }
}
