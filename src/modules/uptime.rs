use crate::color::Rule;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Uptime {
    format: String,
    border: bool,
    height: usize,
    padding: usize,
    alignment: String,
    rule: Vec<Rule>,
}

#[derive(Default)]
struct Time {
    days: String,
    hours: String,
    minutes: String,
    seconds: String,
}
impl Module for Uptime {
    fn format(&self) -> String {
        let time = match sys_info::boottime() {
            Ok(info) => {
                let mut time = info.tv_sec;
                let days = (time / (24 * 3600)).to_string();
                time %= 24 * 3600;
                let hours = (time / 3600).to_string();
                time %= 3600;
                let minutes = (time / 60).to_string();
                let seconds = (time % 60).to_string();
                Time {
                    days,
                    hours,
                    minutes,
                    seconds,
                }
                // self.format
            }
            Err(_) => Time::default(),
        };
        if !self.rule.is_empty() {
            self.rule.iter().fold(self.format.clone(), |acc, rule| {
                acc.replace(&rule.id, &rule.get_colored())
            })
        } else {
            self.format.clone()
        }
        .replace("$d", &time.days)
        .replace("$h", &time.hours)
        .replace("$m", &time.minutes)
        .replace("$s", &time.seconds)
        .add_padding(&self.padding)
        .add_border(&self.height, &self.alignment, self.border)
    }
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            format: String::from(
                "\u{1b}[38;2;255;255;255;49;1mUptime:\u{1b}[0m $d days, $h hours, $m minutes",
            ),
            border: false,
            padding: 0,
            height: 0,
            alignment: String::from("left"),
            rule: vec![],
        }
    }
}
