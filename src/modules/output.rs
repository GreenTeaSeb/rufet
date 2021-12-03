use crate::utils::Module;
use toml::Value;
pub struct Output {
    data: String,
    logo: String,
}

impl Module for Output {}
impl Output {
    pub fn new(data_in: String, logo_in: String, config_data: &Value, config_logo: &Value) -> Self {
        Self {
            data: Self::with_border(Some(config_data), data_in, true),
            logo: Self::with_border(Some(config_logo), logo_in, true),
        }
    }
}
impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = std::cmp::max(self.logo.lines().count(), self.data.lines().count());
        let mut data_lines = self.data.lines();
        let mut logo_lines = self.logo.lines();
        let mut output = String::default();
        let longest_string = self
            .logo
            .lines()
            .map(|x| x.chars().count())
            .max()
            .unwrap_or(1)
            .max(1);
        (0..l).for_each(|_| {
            let data_line = data_lines.nth(0).unwrap_or(" ");
            let logo_line = logo_lines.nth(0).unwrap_or(" ");
            output = format!(
                "{}{:0widthL$}{}\n",
                output,
                logo_line,
                data_line,
                widthL = longest_string
            );
        });
        write!(f, "{}", output)
    }
}
