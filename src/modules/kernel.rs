use crate::utils::Module;
pub struct Kernel {}
impl Module for Kernel {
    fn get_val() -> Option<String> {
        sys_info::os_release().ok()
    }
}
