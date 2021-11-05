use crate::utils::Module;
pub struct Uptime {}
impl Module for Uptime {
    fn get_val() -> String {
        match sys_info::boottime() {
            Ok(info) => {
                let mut time = info.tv_sec;
                let days = time / (24 * 3600);
                time %= 24 * 3600;
                let hours = time / 3600;
                time %= 3600;
                let minutes = time / 60;
                format!("{} days, {:02}:{:02}", days, hours, minutes)
            }
            Err(_) => "".to_string(),
        }
    }
    fn default_format() -> &'static str {
        "Uptime: $value\n"
    }
}
