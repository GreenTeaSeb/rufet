use crate::utils::Module;
pub struct Memory {}
impl Module for Memory {
    fn get_val() -> Option<String> {
        match sys_info::mem_info() {
            Ok(mem) => Some(format!(
                "{:.4}",
                (mem.total as f32 / (1024_f32.powf(2.0))).to_string()
            )),
            _ => Some(String::default()),
        }
    }
}
