use crate::borders::Border;
use crate::color::Rule;
use crate::modules::*;
use crate::utils::*;
use serde::Deserialize;

use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Data {
    format: String,
    padding: usize,
    margin: usize,
    height: usize,
    border: Border,
    alignment: String,
    hostname: hostname::Hostname,
    memory: memory::Memory,
    kernel: kernel::Kernel,
    os: os::Os,
    uptime: uptime::Uptime,
    rule: Vec<Rule>,
}
impl Module for Data {
    fn format(&self) -> String {
        let map = BTreeMap::from([
            ("$hostname", self.hostname.format()),
            ("$memory", self.memory.format()),
            ("$kernel", self.kernel.format()),
            ("$os", self.os.format()),
            ("$uptime", self.uptime.format()),
        ]);
        let mut output = if !self.rule.is_empty() {
            self.rule
                .iter()
                .map(|rule| self.format.replace(&rule.id, &rule.get_colored()))
                .collect::<String>()
        } else {
            self.format.clone()
        };
        if output.contains("$all") {
            output = output.replace(
                "$all",
                &map.keys()
                    .filter(|key| !output.contains(&key.to_string()))
                    .cloned()
                    .collect::<String>(),
            );
        }
        map.iter().for_each(|(k, v)| output = output.replace(k, v));
        let formated = output
            .add_margin(&self.padding)
            .add_height(&self.height)
            .align(&self.alignment);
        self.border.add_border(&formated).add_margin(&self.margin)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            format: String::from("$all"),
            margin: 0,
            padding: 9,
            height: 25,
            border: Border {
                visible: true,
                ..Border::default()
            },
            alignment: "left".to_string(),
            hostname: hostname::Hostname::default(),
            memory: memory::Memory::default(),
            kernel: kernel::Kernel::default(),
            os: os::Os::default(),
            uptime: uptime::Uptime::default(),
            rule: vec![],
        }
    }
}
