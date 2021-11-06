use crate::utils::Module;
pub struct Os {}
impl Module for Os {
    fn get_val() -> Option<String> {
        match sys_info::linux_os_release() {
            Ok(info) => info.pretty_name,
            Err(_) => None,
        }
    }
}
