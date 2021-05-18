use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

use indexmap::IndexMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AliasBody {
    String(String),
    AliasComplex {
        command: String,
        // dependencies: Option<Vec<String>>,
        or: Option<String>,
    },
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// #[serde(untagged)]
// pub enum DependencyBody {
//     String(String),
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawConfig {
    pub path: Option<Vec<String>>,
    pub alias: Option<IndexMap<String, AliasBody>>,
    pub env: Option<IndexMap<String, String>>,
    pub dependencies: Option<IndexMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub path: Vec<String>,
    pub alias: IndexMap<String, String>,
    pub env: IndexMap<String, String>,
    pub dependencies: IndexMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NuConfig {
    pub startup: Vec<String>,
    pub path: Vec<String>,
    pub env: IndexMap<String, String>,
}

impl Config {
    pub fn install(&self, name: &str) {
        if let Some(command) = self.dependencies.get(name) {
            if let Some((first, args)) = command
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
                .split_first()
            {
                let mut child = std::process::Command::new(&first)
                    .args(args)
                    // .stdout(std::process::Stdio::null())
                    // .stderr(std::process::Stdio::null())
                    .spawn()
                    .expect(&format!("Failed to excuete {}", command));

                if child
                    .wait()
                    .expect(&format!("Failed to excute {}", command))
                    .success()
                {
                    println!("Success to install {}: `{}`", name, command);
                }
            }
        } else {
            eprintln!("Not found {}", name)
        }
    }

    pub fn install_bg(&self, name: &str) {
        if let Some(command) = self.dependencies.get(name) {
            if let Some((first, args)) = command
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>()
                .split_first()
            {
                let mut child = std::process::Command::new(&first)
                    .args(args)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn()
                    .expect(&format!("Failed to excuete {}", command));
            }
        } else {
            eprintln!("Not found {}", name)
        }
    }
}

impl From<RawConfig> for Config {
    fn from(raw: RawConfig) -> Self {
        let alias = raw
            .alias
            .unwrap_or(IndexMap::new())
            .into_iter()
            .map(|(name, body)| match body {
                // AliasBody::Simple(s) => Some((name, s)),
                // AliasBody::Complex(cc) => None,
                AliasBody::String(s) => Some((name, s)),
                AliasBody::AliasComplex {
                    command,
                    // dependencies,
                    or,
                } => match (command.split_whitespace().collect::<Vec<_>>().first(), or) {
                    (Some(first), _) if which::which(first).is_ok() => Some((name, command)),
                    (_, Some(or_command)) => Some((name, or_command)),
                    _ => None,
                },
            })
            .filter_map(|e| e)
            .collect::<IndexMap<_, _>>();

        Self {
            path: raw.path.unwrap_or(Vec::new()),
            alias,
            env: raw.env.unwrap_or(IndexMap::new()),
            dependencies: raw.dependencies.unwrap_or(IndexMap::new()),
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
                    p = p.replace(format!("${}", k).as_str(), v.as_str())
                }
                p
            })
            .collect::<Vec<_>>();

        Self { startup, path, env }
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
