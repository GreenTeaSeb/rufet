use crate::utils::Module;
pub struct Hostname {}
impl Module for Hostname {
    fn get_val() -> Option<String> {
        sys_info::hostname().ok()
    }
}
