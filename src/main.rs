mod modules;
use crate::modules::*;
use serde::Deserialize;
use utils::Module;

#[derive(Deserialize, Default)]
#[serde(default)]
struct Rufet {
    data: data::Data,
    logo: logo::Logo,
}

impl Rufet {
    fn format(&self, first: &String, second: &String) -> String {
        let l = std::cmp::max(first.lines().count(), second.lines().count());
        let mut data_lines = second.lines();
        let mut logo_lines = first.lines();
        let mut output = String::default();
        let text_reg = regex::Regex::new(r"\u001b[^m]*?m").unwrap();
        let longest_string = first
            .lines()
            .map(|x| {
                if text_reg.is_match(x) == true {
                    text_reg.replace_all(x, "").chars().count()
                } else {
                    x.chars().count()
                }
            })
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
        output
    }
}

fn get_config() -> Result<String, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(env!("HOME").to_owned() + "/.config/rufet.toml")?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rufet: Rufet = match get_config() {
        Ok(x) => toml::from_str(&x).unwrap(),
        Err(_) => Rufet::default(),
    };
    let output = rufet.format(&rufet.logo.format(), &rufet.data.format());
    println!("{}", &output);
    Ok(())
}
