use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Kernel {
    format: String,
    border: bool,
    height: usize,
    padding: usize,
}
impl Module for Kernel {
    fn format(&self) -> String {
        let output = default_format(&self.format, &self.get_val());
        if self.border == false {
            return output;
        }
        add_border(output, self.padding, self.height, "center")
    }
    fn get_val(&self) -> String {
        sys_info::os_release().unwrap_or_default()
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self {
            format: String::from("{Kernel}(224, 16, 71): $value\n"),
            border: false,
            padding: 0,
            height: 0,
        }
    }
}
