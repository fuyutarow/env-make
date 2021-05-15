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
    pub aliases: Vec<Alias>,
}

impl Config {
    fn to_nu(&self) -> anyhow::Result<Vec<String>> {
        let mut lines = Vec::<String>::new();
        lines.push(String::from("startup = ["));

        for alias in self.clone().aliases {
            lines.push(format!("    {},", alias.to_nu()));
        }
        lines.push(String::from("]"));

        lines.push(format!(
            r##"
path = [
"{home_dir}/.linuxbrew/bin",
"{home_dir}/.linuxbrew/sbin",
]

[env]
HOMEBREW_PREFIX = "{home_dir}/.linuxbrew"
HOMEBREW_CELLAR = "{home_dir}/.linuxbrew/Cellar"
HOMEBREW_REPOSITORY = "{home_dir}/.linuxbrew/Homebrew"
# PATH = {home_dir}/.linuxbrew/bin:{home_dir}/.linuxbrew/sbin${{PATH+:$PATH}}"
MANPATH = "{home_dir}/.linuxbrew/share/man${{MANPATH+:$MANPATH}}:"
INFOPATH = "{home_dir}/.linuxbrew/share/info:${{INFOPATH:-}}"
"##,
            home_dir = dirs::home_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
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

impl From<RawConfig> for Config {
    fn from(raw_config: RawConfig) -> Self {
        let aliases = raw_config
            .alias
            .into_iter()
            .map(|(name, body)| Alias { name, body })
            .collect::<Vec<Alias>>();
        Self { aliases }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RawConfig {
    alias: HashMap<String, String>,
}
