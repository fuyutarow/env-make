use cli::{Alias, Config};
use std::io::prelude::*;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let config = Config {
        aliases: vec![
            Alias {
                name: String::from("p"),
                body: String::from("cat"),
            },
            Alias {
                name: String::from(","),
                body: String::from("cd"),
            },
            Alias {
                name: String::from("l"),
                body: String::from("ls -l"),
            },
            Alias {
                name: String::from("g"),
                body: String::from("git"),
            },
        ],
    };
    config.write_nu()?;
    Ok(())
}
