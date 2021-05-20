use std::path::PathBuf;

use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};

pub trait ShConfig {
    fn print(&self) -> anyhow::Result<()>;

    fn write(&self, fpath: PathBuf) -> anyhow::Result<()>;
}

mod bash;
pub use bash::BashConfig;
mod nushell;
pub use nushell::NuConfig;

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
                let _child = std::process::Command::new(&first)
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
