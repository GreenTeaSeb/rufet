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
    height: usize,
    border: bool,
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
        let mut output = self.format.clone();
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
        output
            .add_padding(&self.padding)
            .add_border(&self.height, &self.alignment, self.border)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            format: String::from("$all"),
            padding: 5,
            height: 25,
            border: true,
            alignment: "left".to_string(),
            hostname: hostname::Hostname::default(),
            memory: memory::Memory::default(),
            kernel: kernel::Kernel::default(),
            os: os::Os::default(),
            uptime: uptime::Uptime::default(),
            rule: vec![Rule::default()],
        }
    }
}
