use cli::Alias;
use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone)]
struct Config {
    aliases: Vec<Alias>,
}

impl Config {
    fn to_nu(&self) -> anyhow::Result<String> {
        use std::fmt::Write;
        let mut s = String::new();
        writeln!(s, r#"path = "~/.linuxbrew/bin/""#);
        writeln!(s, "startup = [");
        for alias in self.clone().aliases {
            writeln!(s, "    {},", alias.to_nu());
        }
        writeln!(s, "]");
        Ok(s)
    }

    fn print_nu(&self) {
        if let Ok(s) = self.to_nu() {
            println!("{}", s);
        }
    }

    fn write_nu(&self) -> anyhow::Result<String> {
        let s = self.to_nu()?;
        let fpath = dirs::config_dir()
            .expect("expected file path")
            .join("nu/config.toml");
        let mut f = std::fs::File::create(&fpath).expect("failed to create file");
        f.write_all(&s.as_bytes()).expect("failed to write file");
        Ok(s)
    }
}

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
