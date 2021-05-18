use std::path::{Path, PathBuf};

use parse_display::{Display, FromStr};
use structopt::StructOpt;

mod lib;
use lib::{BashConfig, Config, NuConfig, RawConfig, ShConfig};

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
        #[structopt(parse(from_os_str), short, long = "file")]
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
        ///
        #[structopt()]
        name: String,

        /// target config file
        #[structopt(parse(from_os_str), short, long = "file")]
        fpath: PathBuf,

        /// output file path
        #[structopt(parse(from_os_str), short)]
        out: PathBuf,

        ///
        #[structopt(short, long)]
        background: bool,
    },
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
                To::Nu => dirs::config_dir()
                    .expect("expected file path")
                    .join("nu/config.toml"),
            });

            let sh = match to {
                To::Bash | To::Zsh => BashConfig::from(config),
                To::Nu => NuConfig::from(config),
            };

            if replace {
                sh.write(out);
            } else {
                sh.print();
            }
        }
        Opt::Install {
            name,
            fpath,
            out,
            background,
        } => {
            let content = std::fs::read_to_string(fpath).expect("Unable to read file");
            let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
            let config = Config::from(raw);

            if background {
                config.install_bg(&name)
            } else {
                config.install(&name)
            }
        }
    }
}
