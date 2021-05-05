use cli::{Config, RawConfig};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::prelude::*;

fn main() -> anyhow::Result<()> {
    let fpath = "examples/aliases.toml";

    let mut f = std::fs::File::open(fpath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let raw_config = toml::from_str::<RawConfig>(&contents).expect("failed to parse config file");
    let config = Config::from(raw_config);
    config.write_nu()?;
    Ok(())
}
