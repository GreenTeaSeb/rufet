use crate::utils::Module;
pub struct Os {}
impl Module for Os {
    fn get_val() -> String {
        crate::modules::utils::parse(
            "NAME",
            "=",
            &std::fs::read_to_string("/etc/os-release").unwrap_or_default(),
        )
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
    }
    fn default_format() -> &'static str {
        "Os: $value\n"
    }
}
