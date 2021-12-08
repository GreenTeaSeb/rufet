use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Hostname {
    format: String,
    border: bool,
    height: usize,
    padding: usize,
}
impl Module for Hostname {
    fn format(&self) -> String {
        let output = default_format(&self.format, &self.get_val());
        if self.border == false {
            return output;
        }
        add_border(output, self.height, "center")
    }
    fn get_val(&self) -> String {
        sys_info::hostname().unwrap_or_default()
    }
}

impl Default for Hostname {
    fn default() -> Self {
        Self {
            format: String::from("{Hostname}(bold): $value\n"),
            border: false,
            padding: 0,
            height: 0,
        }
    }
}
