use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawConfig {
    pub path: Option<Vec<String>>,
    pub alias: Option<HashMap<String, String>>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub path: Vec<String>,
    pub alias: HashMap<String, String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NuConfig {
    pub startup: Vec<String>,
    pub path: Vec<String>,
    pub env: HashMap<String, String>,
}

impl From<RawConfig> for Config {
    fn from(raw: RawConfig) -> Self {
        Self {
            path: raw.path.unwrap_or(Vec::new()),
            alias: raw.alias.unwrap_or(HashMap::new()),
            env: raw.env.unwrap_or(HashMap::new()),
        }
    }
}

impl From<Config> for NuConfig {
    fn from(config: Config) -> Self {
        let startup = config
            .alias
            .into_iter()
            .map(|(k, v)| format!("alias {} = {}", k, v))
            .collect::<Vec<String>>();

        Self {
            startup,
            env: config.env,
            path: config.path,
        }
    }
}

impl NuConfig {
    pub fn print(&self) {
        match toml::to_string(&self) {
            Ok(toml) => {
                println!("{}", toml);
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        };
    }

    pub fn write(&self) {
        let nu_config_fpath = dirs::config_dir()
            .expect("expected file path")
            .join("nu/config.toml");
        if let Ok(toml) = toml::to_string(&self) {
            match std::fs::write(&nu_config_fpath, toml) {
                Ok(_) => println!(
                    "{} was updated",
                    nu_config_fpath.into_os_string().into_string().unwrap()
                ),
                Err(err) => eprintln!("{}", err),
            }
        };
    }
}

impl Config {
    // pub fn write_nu(&self) -> anyhow::Result<std::path::PathBuf> {
    //     let lines = self.to_nu()?;
    //     let fpath = dirs::config_dir()
    //         .expect("expected file path")
    //         .join("nu/config.toml");

    //     let s = lines.join("\n");
    //     std::fs::write(&fpath, s)?;
    //     Ok(fpath)
    // }
}
