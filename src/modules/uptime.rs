use crate::utils::Module;
use crate::Value;
pub struct Uptime {}
impl Module for Uptime {
    fn print(config: Option<&Value>, default: &'static str) -> String {
        let form = Self::get_from(config, "format", default);
        match sys_info::boottime() {
            Ok(info) => {
                let mut time = info.tv_sec;
                let days = (time / (24 * 3600)).to_string();
                time %= 24 * 3600;
                let hours = (time / 3600).to_string();
                time %= 3600;
                let minutes = (time / 60).to_string();
                let seconds = (time % 60).to_string();
                let output = form
                    .replace("$d", &days)
                    .replace("$h", &hours)
                    .replace("$m", &minutes)
                    .replace("$s", &seconds);
                Self::with_border(config, output, false)
            }
            Err(_) => String::from("UNKOWN UPTIME"),
        }
    }
}
