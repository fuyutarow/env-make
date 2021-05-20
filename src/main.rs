use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;
use parse_display::{Display, FromStr};
use structopt::StructOpt;

mod lib;
use lib::{BashConfig, Config, NuConfig, RawConfig, ShConfig};

fn touch(path: &Path) -> std::io::Result<()> {
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

lazy_static! {
    static ref CONFIG_PATH: String = {
        let path = dirs::settings_dir()
            .unwrap()
            .join("env-make")
            .join("config.toml");
        std::fs::create_dir_all(path.parent().unwrap());
        touch(&path);
        path.to_str().unwrap().to_string()
    };
}

#[derive(Debug, Display, FromStr)]
#[display(style = "lowercase")]
enum To {
    Bash,
    Zsh,
    Nu,
}

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "build")]
    Build {
        /// target config file
        #[structopt(parse(from_os_str), short, long = "file", default_value=&CONFIG_PATH)]
        fpath: PathBuf,

        /// output file path
        #[structopt(parse(from_os_str), short)]
        out: Option<PathBuf>,

        /// target config file [possible values: bash, zsh, nu]
        #[structopt(short, long, default_value = "bash")]
        to: To,

        ///
        #[structopt(short, long)]
        replace: bool,
    },
    #[structopt(name = "install")]
    Install {
        /// dependency
        #[structopt()]
        dependency: Option<String>,

        /// all
        #[structopt(short, long)]
        all: bool,

        /// target config file
        #[structopt(parse(from_os_str), short, long = "file", default_value=&CONFIG_PATH)]
        fpath: PathBuf,

        ///
        #[structopt(short, long)]
        background: bool,
    },
    #[structopt(name = "path")]
    Path {},
}

fn main() {
    match Opt::from_args() {
        Opt::Build {
            fpath,
            to,
            out,
            replace,
        } => {
            let content = std::fs::read_to_string(fpath).expect("Unable to read file");
            let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
            let config = Config::from(raw);

            let out = out.unwrap_or(match to {
                To::Bash => dirs::home_dir()
                    .expect("expected file path")
                    .join(".bashrc"),
                To::Zsh => dirs::home_dir().expect("expected file path").join(".zshrc"),
                To::Nu => directories::ProjectDirs::from("org", "nushell", "nu")
                    .expect("expected file path")
                    .config_dir()
                    .join("config.toml"),
            });

            match to {
                To::Bash | To::Zsh => {
                    let sh = BashConfig::from(config);
                    if replace {
                        sh.write(out);
                    } else {
                        sh.print();
                    }
                }
                To::Nu => {
                    let sh = NuConfig::from(config);
                    if replace {
                        sh.write(out);
                    } else {
                        sh.print();
                    }
                }
            };
        }
        Opt::Install {
            dependency,
            all,
            fpath,
            background,
        } => {
            let content = std::fs::read_to_string(fpath).expect("Unable to read file");
            let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
            let config = Config::from(raw);

            match (dependency, all) {
                (Some(name), _) => {
                    if background {
                        config.install_bg(&name)
                    } else {
                        config.install(&name)
                    };
                }
                (_, true) => {
                    for name in config.dependencies.keys() {
                        if background {
                            config.install_bg(&name)
                        } else {
                            config.install(&name)
                        };
                    }
                }
                _ => {
                    println!("`env-make install` requires args or --all option.")
                }
            };
        }
        Opt::Path {} => {
            println!("{}", CONFIG_PATH.to_string());
        }
    }
}
