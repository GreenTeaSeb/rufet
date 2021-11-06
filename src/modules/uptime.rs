use crate::utils::*;
use crate::Value;
pub struct Uptime {}
impl Module for Uptime {
    fn print(config: Option<&Value>, default: &'static str) -> String {
        let form = Self::get_format(config, default);
        match sys_info::boottime() {
            Ok(info) => {
                let mut time = info.tv_sec;
                let days = (time / (24 * 3600)).to_string();
                time %= 24 * 3600;
                let hours = (time / 3600).to_string();
                time %= 3600;
                let minutes = (time / 60).to_string();

                form.replace("$d", &days)
                    .replace("$h", &hours)
                    .replace("$m", &minutes)
            }
            Err(_) => String::from("unknown boot"),
        }
    }
}
