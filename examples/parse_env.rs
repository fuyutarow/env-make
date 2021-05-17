use std::collections::HashMap;
use std::io::prelude::*;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use cli::Config;
use cli::NuConfig;
use cli::RawConfig;

fn main() {
    let fpath = "examples/env.toml";
    let content = std::fs::read_to_string(fpath).expect("Unable to read file");
    let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
    dbg!(&raw);

    let config = Config::from(raw);
    dbg!(&config);

    let nu = NuConfig::from(config);
    dbg!(&nu);
    nu.write();

    // dbg!(nu);

    // dbg!(&config);

    // match toml::to_string(&config) {
    //     Ok(toml) => {
    //         println!("{}", toml);
    //     }
    //     Err(err) => {
    //         eprintln!("{}", err)
    //     }
    // };
}
