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
    fn to_nu(&self) -> anyhow::Result<String> {
        let mut s = String::new();
        writeln!(s, "startup = [");
        for alias in self.clone().aliases {
            writeln!(s, "    {},", alias.to_nu());
        }
        writeln!(s, "]");
        writeln!(
            s,
            r##"
path = [
    "/home/fuyutarow/.linuxbrew/bin",
    "/home/fuyutarow/.linuxbrew/sbin",
]

[env]
HOMEBREW_PREFIX = "/home/fuyutarow/.linuxbrew"
HOMEBREW_CELLAR = "/home/fuyutarow/.linuxbrew/Cellar"
HOMEBREW_REPOSITORY = "/home/fuyutarow/.linuxbrew/Homebrew"
# PATH = "/home/fuyutarow/.linuxbrew/bin:/home/fuyutarow/.linuxbrew/sbin${{PATH+:$PATH}}"
MANPATH = "/home/fuyutarow/.linuxbrew/share/man${{MANPATH+:$MANPATH}}:"
INFOPATH = "/home/fuyutarow/.linuxbrew/share/info:${{INFOPATH:-}}"
        "##
        );
        Ok(s)
    }

    pub fn print_nu(&self) {
        if let Ok(s) = self.to_nu() {
            println!("{}", s);
        }
    }

    pub fn write_nu(&self) -> anyhow::Result<String> {
        let s = self.to_nu()?;
        let fpath = dirs::config_dir()
            .expect("expected file path")
            .join("nu/config.toml");
        let mut f = std::fs::File::create(&fpath).expect("failed to create file");
        f.write_all(&s.as_bytes()).expect("failed to write file");
        Ok(s)
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
