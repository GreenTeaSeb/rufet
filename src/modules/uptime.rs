use crate::utils::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Uptime {
    format: String,
    border: bool,
    height: usize,
    padding: usize,
}
impl Module for Uptime {
    fn format(&self) -> String {
        match sys_info::boottime() {
            Ok(info) => {
                let mut time = info.tv_sec;
                let days = (time / (24 * 3600)).to_string();
                time %= 24 * 3600;
                let hours = (time / 3600).to_string();
                time %= 3600;
                let minutes = (time / 60).to_string();
                let seconds = (time % 60).to_string();
                let output = self
                    .format
                    .replace("$d", &days)
                    .replace("$h", &hours)
                    .replace("$m", &minutes)
                    .replace("$s", &seconds);
                if self.border == false {
                    return to_colored(&output);
                }
                add_border(to_colored(&output), self.padding, self.height, "center")
            }
            Err(_) => String::from("UNKOWN UPTIME"),
        }
    }
    fn get_val(&self) -> String {
        sys_info::hostname().unwrap_or_default()
    }
}

impl Default for Uptime {
    fn default() -> Self {
        Self {
            format: String::from("{Uptime}(224, 16, 71): $d days, $h hours, $m minutes\n"),
            border: false,
            padding: 0,
            height: 0,
        }
    }
}
