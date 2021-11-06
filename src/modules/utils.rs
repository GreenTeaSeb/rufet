use crate::Value;
pub fn parse(key: &str, delim: &str, string: &str) -> std::io::Result<String> {
    let mut line = string.lines().find(|n| n.contains(key));
    if line.is_some() {
        Ok(line
            .take()
            .unwrap_or_default()
            .split(delim)
            .nth(1)
            .unwrap_or_default()
            .to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "no key found",
        ))
    }
}

pub fn remove_trailing<'a>(string: &'a str, suffix: &str) -> &'a str {
    string.strip_suffix(suffix).unwrap_or(string)
}
//
pub struct Data {
    pub value: String,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.value)
    }
}

pub trait Module {
    fn print(config: Option<&Value>, default: &'static str) -> String {
        Self::get_format(config, default).replace("$value", &Self::get_val())
    }
    fn get_format(config: Option<&Value>, default: &'static str) -> String {
        match config {
            Some(con) => match con.get("format") {
                Some(con) => con.as_str().get_or_insert(default).to_string(),
                None => String::from(default),
            },
            None => default.to_string(),
        }
    }
    fn get_val() -> String {
        String::from("aa")
    }
}
