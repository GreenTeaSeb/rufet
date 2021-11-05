use crate::utils::Module;
pub struct Hostname {}
impl Module for Hostname {
    fn get_val() -> String {
        let string = std::fs::read_to_string("/etc/hostname").unwrap_or_default();
        crate::modules::utils::remove_trailing(&string, "\n").to_string()
    }
    fn default_format() -> &'static str {
        "Hostname: $value\n"
    }
}
