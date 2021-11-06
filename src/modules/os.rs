use crate::utils::Module;
pub struct Os {}
impl Module for Os {
    fn get_val() -> String {
        match crate::modules::utils::parse(
            "NAME",
            "=",
            &std::fs::read_to_string("/etc/os-release").unwrap_or_default(),
        ) {
            Ok(x) => x.trim_matches('"').to_string(),
            Err(_) => String::from("os-release not found"),
        }
    }
}
