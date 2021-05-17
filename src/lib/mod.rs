use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Alias {
    pub name: String,
    pub body: String,
}

impl Alias {
    pub fn to_bash(&self) -> String {
        format!(r#"alias {} = "{}""#, self.name, self.body)
    }

    pub fn to_nu(&self) -> String {
        format!(r#""alias {} = {}""#, self.name, self.body)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub path: Vec<String>,
    pub aliases: Vec<Alias>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NuConfig {
    pub startup: Vec<String>,
    pub path: Vec<String>,
    pub env: HashMap<String, String>,
}

impl From<Config> for NuConfig {
    fn from(config: Config) -> Self {
        let startup = config
            .aliases
            .into_iter()
            .map(|alias| format!("alias {} = {}", alias.name, alias.body))
            .collect::<Vec<String>>();

        Self {
            startup,
            env: config.env,
            path: config.path,
        }
    }
}

impl Config {
    fn to_nu(&self) -> anyhow::Result<Vec<String>> {
        let mut lines = Vec::<String>::new();
        lines.push(String::from("startup = ["));

        for alias in self.clone().aliases {
            lines.push(format!("    {},", alias.to_nu()));
        }
        lines.push(String::from("]"));

        // "{home_dir}/.linuxbrew/bin",
        // "{home_dir}/.linuxbrew/sbin",

        lines.push(format!(
            r##"
path = [
"$HOMEBREW_PREFIX/bin",
"$HOMEBREW_PREFIX/sbin",
]
"##,
            // home_dir = dirs::home_dir()
            //     .unwrap()
            //     .into_os_string()
            //     .into_string()
            //     .unwrap()
        ));
        Ok(lines)
    }

    pub fn print_nu(&self) {
        if let Ok(lines) = self.to_nu() {
            for line in lines {
                println!("{}", line);
            }
        }
    }

    pub fn write_nu(&self) -> anyhow::Result<std::path::PathBuf> {
        let lines = self.to_nu()?;
        let fpath = dirs::config_dir()
            .expect("expected file path")
            .join("nu/config.toml");

        let s = lines.join("\n");
        std::fs::write(&fpath, s)?;
        Ok(fpath)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawConfig {
    alias: HashMap<String, String>,
}
