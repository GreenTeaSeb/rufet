use toml::Value;
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

pub struct Data<T: Module> {
    pub value: String,
    pub format: String,

    pub module: T,
}

impl<T: Module> Data<T> {
    pub fn new(t: T, config: Value) -> Self {
        Data {
            format: config["format"].as_str().unwrap().to_string(),
            value: T::get_val(),
            module: t,
        }
    }
    pub fn print(&self) -> String {
        self.format.replace("$value", &self.value)
    }
}

pub trait Module {
    fn get_val() -> String;
}
