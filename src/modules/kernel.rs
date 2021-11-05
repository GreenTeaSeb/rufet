use crate::utils::Module;
pub struct Kernel {}
impl Module for Kernel {
    fn get_val() -> String {
        std::fs::read_to_string("/proc/version").unwrap_or_default()
    }
    fn default_format() -> &'static str {
        "Kernel: $value\n"
    }
}
