mod modules;
use crate::modules::*;
use color::AnsiExt;
use dirs::config_dir;
use serde::Deserialize;
use utils::Module;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct Rufet {
    data: data::Data,
    logo: logo::Logo,
}

impl Rufet {
    fn format(&self, first: &str, second: &str) -> String {
        let l = std::cmp::max(first.lines().count(), second.lines().count());
        let mut data_lines = second.lines();
        let mut logo_lines = first.lines();
        let mut output = String::default();
        let longest_string = first
            .lines()
            .map(|x| x.to_string().remove_ansi().chars().count())
            .max()
            .unwrap_or(1)
            .max(1);
        (0..l).for_each(|_| {
            let data_line = data_lines.next().unwrap_or(" ");
            let logo_line = logo_lines.next().unwrap_or(" ");
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
    let file = config_dir().ok_or("no")?.join("rufet/config.toml");
    Ok(std::fs::read_to_string(&*file.to_string_lossy())?)
}

fn main() {
    let rufet: Rufet = match get_config() {
        Ok(x) => toml::from_str(&x).unwrap(),
        Err(_) => Rufet::default(),
    };

    let output = rufet.format(&rufet.logo.format(), &rufet.data.format());
    println!("{}", &output);
}
