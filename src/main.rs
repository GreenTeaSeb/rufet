mod modules;
use crate::modules::utils::Module;
use crate::modules::*;
use std::collections::BTreeMap;
use std::env;
use toml::Value;

fn get_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(env!("HOME").to_owned() + "/.config/rufet.toml")?;
    Ok(toml::from_str(&config)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match get_config() {
        Ok(con) => con,
        Err(_) => toml::from_str("[default]")?,
    };

    let mut format = match config.get("format") {
        Some(con) => con.as_str().get_or_insert("$all").to_string(),
        None => String::from("$all"),
    };

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
    ]);

    if format.contains("$all") {
        format = format.replace(
            "$all",
            &map.keys()
                .filter(|key| !format.contains(&key.to_string()))
                .cloned()
                .collect::<String>(),
        );
    }
    map.iter().for_each(|(k, v)| format = format.replace(k, v));

    Ok(println!("{}", format))
}
