use crate::utils::Module;
pub struct Kernel {}
impl Module for Kernel {
    fn get_val() -> String {
        match sys_info::os_release() {
            Ok(info) => info.to_string(),
            Err(_) => "".to_string(),
        }
    }
    fn default_format() -> &'static str {
        "Kernel: $value\n"
    }
}
