use crate::modules::*;
use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Logo {
    format: String,
    padding: usize,
    height: usize,
    border: bool,
    hostname: hostname::Hostname,
    memory: memory::Memory,
}
impl Module for Logo {
    fn format(&self) -> String {
        if self.border == false {
            return self.format.clone();
        }
        add_border(self.format.clone(), self.padding, self.height, "center")
    }
}

impl Default for Logo {
    fn default() -> Self {
        Self {
            format: String::default(),
            padding: 0,
            height: 0,
            border: true,
            hostname: hostname::Hostname::default(),
            memory: memory::Memory::default(),
        }
    }
}
