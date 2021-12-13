#![allow(dead_code)]
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Rule {
    pub id: String,
    pub text: String,
    pub exec: String,
    pub color: String,
    pub background: String,
    pub effects: Vec<String>,
}

impl Rule {
    pub fn get_colored(&self) -> String {
        self.format_to_ansi(
            &self.get_foreground(),
            &self.get_background(),
            &self.get_effects(),
        )
    }

    fn format_to_ansi(&self, foreground: &str, background: &str, effects: &str) -> String {
        format!(
            "\u{1b}[{fg}{bg}{effects}m{data}\u{1b}[0m",
            fg = foreground,
            bg = background,
            data = self.get_data(),
            effects = effects
        )
    }
    fn parse(color: &str) -> Option<String> {
        if color.is_empty() {
            return None;
        }
        if color.trim().starts_with('#') {
            if let Some(hex) = color.get(1..) {
                let hex = if hex.chars().count() == 3 {
                    hex.chars()
                        .map(|x| x.to_string().repeat(2))
                        .collect::<String>()
                } else {
                    hex.to_string()
                };
                let hex_b = usize::from_str_radix(&hex, 16).unwrap_or_default();

                let rgb = (hex_b >> 16).to_string()
                    + ";"
                    + &((hex_b >> 8) & 0xFF).to_string()
                    + ";"
                    + &(hex_b & 0xFF).to_string();
                return Some(rgb);
            } else {
                return None;
            }
        }; // parse rgb
        Some(color.to_string())
    }
    fn get_data(&self) -> String {
        if !self.exec.is_empty() {
            return String::from_utf8(
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&self.exec)
                    .output()
                    .expect("failed to execute process")
                    .stdout,
            )
            .unwrap_or_default();
        }
        if self.text.is_empty() {
            self.id.clone()
        } else {
            self.text.clone()
        }
    }

    fn get_background(&self) -> String {
        match Rule::parse(&self.background) {
            Some(x) => format!("48;2;{}", x),
            None => "49".to_string(),
        }
    }
    fn get_foreground(&self) -> String {
        match Rule::parse(&self.color) {
            Some(x) => format!("38;2;{};", x),
            None => "39;".to_string(),
        }
    }

    fn get_effects(&self) -> String {
        self.effects
            .iter()
            .map(|effect| match effect.as_str() {
                "bold" => ";1",
                "underline" => ";4",
                _ => "",
            })
            .collect::<String>()
    }
}

fn to_rgb(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|p| p.trim().parse::<u8>().unwrap_or_default())
        .collect::<Vec<u8>>()
}
pub trait AnsiExt {
    fn remove_ansi(&self) -> Self;
}
impl AnsiExt for String {
    /// Goes character by character, when it finds a character that opens up the ansi
    /// code, it changes the state to "open" = 1, then every character that afterwords is not
    /// passed on to the new iterrator, until it comes to the character 'm', upon which it
    /// changes the state to "closed" = 1 and every character afterwords is passed onto the new
    /// iterrator. When it finds another 'm' , it checks whether it is the closing ansi
    /// code character, or just a random m.
    fn remove_ansi(&self) -> Self {
        self.chars()
            .into_iter()
            .scan(0, move |state, elem| {
                if elem.eq(&'\u{1b}') {
                    *state = 1;
                    return Some(None);
                }
                if elem.eq(&'m') {
                    if *state == 1 {
                        *state = 0;
                        return Some(None);
                    }
                    *state = 0;
                    return Some(Some(elem));
                }
                if *state == 0 {
                    Some(Some(elem))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .collect::<String>()
    }
}
