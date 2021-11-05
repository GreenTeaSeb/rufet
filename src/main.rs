mod modules;
use crate::modules::{hostname, kernel, os, utils};
use std::collections::HashMap;
use std::env;
use toml::Value;

fn get_config() -> Result<Value, Box<dyn std::error::Error>> {
    let config = std::fs::read_to_string(env!("HOME").to_owned() + "/.config/rufet.toml")?;
    Ok(toml::from_str(&config)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = match get_config() {
        Ok(con) => con,
        _ => toml::from_str("[default]")?,
    };

    let mut format = match config.get("format") {
        Some(con) => con.as_str().get_or_insert("$all").to_string(),
        _ => String::from("$all"),
    };
    let map = HashMap::from([
        ("$os", utils::Data::new(os::Os {}, config.get("os")).print()),
        (
            "$hostname",
            utils::Data::new(hostname::Hostname {}, config.get("hostname")).print(),
        ),
        (
            "$kernel",
            utils::Data::new(kernel::Kernel {}, config.get("kernel")).print(),
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
    for (key, val) in &map {
        format = format.replace(key, &val);
    }
    Ok(println!("{}", format))
}
