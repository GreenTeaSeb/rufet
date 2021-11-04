mod modules;
use crate::modules::{hostname, kernel, os, utils};
use std::collections::HashMap;
use std::env;
use toml::Value;
fn main() {
    let config = std::fs::read_to_string(env!("HOME").to_owned() + "/.config/rufet.toml")
        .unwrap_or_default();

    let parsed: Value = toml::from_str(&config).unwrap();
    let mut format = parsed["format"].as_str().unwrap().to_string();

    let map = HashMap::from([
        (
            "$hostname",
            utils::Data::new(hostname::Hostname {}, parsed["hostname"].clone()).print(),
        ),
        (
            "$os",
            utils::Data::new(os::Os {}, parsed["os"].clone()).print(),
        ),
        (
            "$kernel",
            utils::Data::new(kernel::Kernel {}, parsed["kernel"].clone()).print(),
        ),
    ]);

    for (key, val) in map {
        format = format.replace(key, &val);
    }
    println!("{}", format);
}
