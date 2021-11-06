use crate::Value;
// pub fn parse(key: &str, delim: &str, string: &str) -> std::io::Result<String> {
//     let mut line = string.lines().find(|n| n.contains(key));
//     if line.is_some() {
//         Ok(line
//             .take()
//             .unwrap_or_default()
//             .split(delim)
//             .nth(1)
//             .unwrap_or_default()
//             .to_string())
//     } else {
//         Err(std::io::Error::new(
//             std::io::ErrorKind::InvalidData,
//             "no key found",
//         ))
//     }
// }
//
// pub fn remove_trailing<'a>(string: &'a str, suffix: &str) -> &'a str {
//     string.strip_suffix(suffix).unwrap_or(string)
// }
//

pub trait Module {
    fn print(config: Option<&Value>, default: &'static str) -> String {
        let default = format!("\x1b[38;2;23;147;209;1m{def}", def = default);
        match &Self::get_val() {
            Some(value) => Self::get_format(config, &default)
                .replace("$value", format!("\x1b[0m{}", value).as_str()),
            None => String::from(""),
        }
    }
    fn get_format(config: Option<&Value>, default: &str) -> String {
        match config {
            Some(con) => match con.get("format") {
                Some(con) => con.as_str().get_or_insert(default).to_string(),
                None => String::from(default),
            },
            None => default.to_string(),
        }
    }
    fn get_val() -> Option<String> {
        None
    }
}
