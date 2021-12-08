use crate::color::to_colored;
use crate::modules::*;
use crate::utils::*;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
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
        let mut formatted = self.format.clone();
        if formatted.contains("$all") {
            formatted = formatted.replace(
                "$all",
                &map.keys()
                    .filter(|key| !formatted.contains(&key.to_string()))
                    .cloned()
                    .collect::<String>(),
            );
        }
        map.iter()
            .for_each(|(k, v)| formatted = formatted.replace(k, v));
        formatted = add_padding(&formatted, self.padding);

        if self.border == false {
            return formatted;
        }
        add_border(formatted, self.height, &self.alignment)
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
        }
    }
}
