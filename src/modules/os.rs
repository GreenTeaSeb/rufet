use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Os {
    format: String,
    border: bool,
    height: usize,
    padding: usize,
}
impl Module for Os {
    fn format(&self) -> String {
        let output = default_format(&self.format, &self.get_val());
        if self.border == false {
            return output;
        }
        add_border(output, self.padding, self.height, "center")
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
            format: String::from("{Os}(224, 16, 71): $value\n"),
            border: false,
            padding: 0,
            height: 0,
        }
    }
}
