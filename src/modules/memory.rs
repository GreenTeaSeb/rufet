use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Memory {
    format: String,
    border: bool,
    height: usize,
    padding: usize,
}
impl Module for Memory {
    fn format(&self) -> String {
        let output = default_format(&self.format, &self.get_val());
        if self.border == false {
            return output;
        }
        add_border(output, self.height, "center")
    }
    fn get_val(&self) -> String {
        match sys_info::mem_info() {
            Ok(mem) => format!(
                "{:.4}",
                (mem.total as f32 / (1024_f32.powf(2.0))).to_string()
            ),
            _ => String::default(),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            format: String::from("{Memory}(bold): $value\n"),
            border: false,
            padding: 0,
            height: 0,
        }
    }
}
