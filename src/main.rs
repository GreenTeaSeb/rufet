mod modules;
use crate::modules::*;
// use colored::*;
use std::collections::BTreeMap;
use std::env;
use toml::Value;
use utils::Module;
fn get_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(env!("HOME").to_owned() + "/.config/rufet.toml")?;
    Ok(toml::from_str(&config)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match get_config() {
        Ok(con) => con,
        Err(_) => toml::from_str("[default]")?,
    };
    let data = config
        .get("data")
        .unwrap_or(&Value::from("default"))
        .clone();
    let logo = config
        .get("logo")
        .unwrap_or(&Value::from("default"))
        .clone();
    let mut data_format = data
        .get("format")
        .unwrap_or(&Value::String("$all".to_string()))
        .as_str()
        .get_or_insert("$all")
        .to_string();
    let logo_format = logo
        .get("logo")
        .unwrap_or(&Value::String(String::default()))
        .as_str()
        .get_or_insert("")
        .to_string();
    let map = BTreeMap::from([
        ("$os", os::Os::print(config.get("os"), "OS: $value\n")),
        (
            "$hostname",
            hostname::Hostname::print(config.get("hostname"), "Hostname: $value\n"),
        ),
        (
            "$kernel",
            kernel::Kernel::print(config.get("get"), "Kernel: $value\n"),
        ),
        (
            "$uptime",
            uptime::Uptime::print(
                config.get("uptime"),
                "Uptime: $d days, $h hours, $m minutes\n",
            ),
        ),
        (
            "$memory",
            memory::Memory::print(config.get("memory"), "RAM: $value\n"),
        ),
    ]);

    if data_format.contains("$all") {
        data_format = data_format.replace(
            "$all",
            &map.keys()
                .filter(|key| !data_format.contains(&key.to_string()))
                .cloned()
                .collect::<String>(),
        );
    }
    map.iter()
        .for_each(|(k, v)| data_format = data_format.replace(k, v));
    let out = output::Output::new(data_format, logo_format, &data, &logo);
    println!("{}", out);
    //
    //     let test = "this is {blue}(blue) and this is {red}(red)";
    //
    //     let reg = regex::Regex::new(r"\{.*?\}\(.*?\)").unwrap();
    //     reg.find_iter(test)
    //         .for_each(|x| println!("{}", to_colored(x.as_str())));
    Ok(())
}

// fn to_colored(input: &str) -> String {
//     let text_reg = regex::Regex::new(r"\{([^)]+)\}").unwrap();
//     dbg!(text_reg.find(input));
//     input.color("blue").to_string()
// }
