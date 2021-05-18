use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::path::PathBuf;

use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

use crate::Config;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BashConfig {
    pub env: IndexMap<String, String>,
    pub path: Vec<String>,
    pub alias: IndexMap<String, String>,
}

impl From<Config> for BashConfig {
    fn from(config: Config) -> Self {
        Self {
            env: config.env,
            path: config.path,
            alias: config.alias,
        }
    }
}

impl BashConfig {
    pub fn to_string(&self) -> anyhow::Result<(String)> {
        let mut s = String::new();
        for (k, v) in self.env.iter() {
            writeln!(&mut s, r#"export {}="{}""#, k, v)?;
        }
        writeln!(&mut s)?;

        for path in self.path.iter() {
            writeln!(&mut s, r#"export PATH="$PATH:{}""#, path)?;
        }
        writeln!(&mut s)?;

        for (k, v) in self.alias.iter() {
            writeln!(&mut s, r#"alias {}="{}""#, k, v)?;
        }
        writeln!(&mut s)?;

        Ok(s)
    }

    pub fn print(&self) -> anyhow::Result<()> {
        let s = self.to_string()?;
        println!("{}", s);
        Ok(())
    }

    pub fn write(&self, fpath: PathBuf) -> anyhow::Result<()> {
        let content = self.to_string()?;
        match std::fs::write(&fpath, content) {
            Ok(_) => println!(
                "{} was updated",
                fpath.into_os_string().into_string().unwrap()
            ),
            Err(err) => eprintln!("{}", err),
        }
        Ok(())
    }
}
