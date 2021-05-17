// use cli::{Alias, Config};
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use cli::Alias;
use cli::Config;
use cli::NuConfig;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::array::IntoIter::new([$($v,)*]))
    };
}

fn main() {
    let path = vec!["$HOMEBREW_PREFIX/bin", "$HOMEBREW_PREFIX/sbin"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

    let aliases = vec![
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
    ];

    let env: HashMap<_, _> = collection! {
        "HOMEBREW_PREFIX".to_string() => "/home/fuyutarow/.linuxbrew".to_string(),
        "HOMEBREW_CELLAR".to_string()=> "/home/fuyutarow/.linuxbrew/Cellar".to_string(),
        "HOMEBREW_REPOSITORY".to_string()=> "/home/fuyutarow/.linuxbrew/Homebrew".to_string() ,
        "MANPATH".to_string() => "/home/fuyutarow/.linuxbrew/share/man${MANPATH+:$MANPATH}:".to_string(),
        "INFOPATH".to_string()=> "/home/fuyutarow/.linuxbrew/share/info:${INFOPATH:-}".to_string(),
    };

    let config = Config { path, aliases, env };

    dbg!(&config);

    match toml::to_string(&config) {
        Ok(toml) => {
            println!("{}", toml);
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    };
}
