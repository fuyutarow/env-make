use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{Config, ShConfig};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NuConfig {
    pub startup: Vec<String>,
    pub path: Vec<String>,
    pub env: IndexMap<String, String>,
}

impl From<Config> for NuConfig {
    fn from(config: Config) -> Self {
        let startup = config
            .alias
            .into_iter()
            .map(|(k, v)| format!("alias {} = {}", k, v))
            .collect::<Vec<String>>();

        let home_dir_s = dirs::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        let env = config
            .env
            .into_iter()
            .map(|(k, v)| (k, v.replace("$HOME", &home_dir_s)))
            .collect::<IndexMap<_, _>>();

        let path = config
            .path
            .into_iter()
            .map(|mut p| {
                for (k, v) in &env {
                    let vv = v.replace("$HOME", &home_dir_s);
                    p = p.replace(format!("${}", k).as_str(), vv.as_str());
                }
                p
            })
            .collect::<Vec<_>>();

        Self { startup, path, env }
    }
}

impl ShConfig for NuConfig {
    fn print(&self) -> anyhow::Result<()> {
        let content = toml::to_string(&self)?;
        println!("{}", content);
        Ok(())
    }

    fn write(&self, fpath: PathBuf) -> anyhow::Result<()> {
        let content = toml::to_string(&self)?;
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
