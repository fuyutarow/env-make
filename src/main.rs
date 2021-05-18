use std::path::{Path, PathBuf};

use parse_display::{Display, FromStr};
use structopt::StructOpt;

mod lib;
use lib::{Config, NuConfig, RawConfig};

#[derive(Debug, Display, FromStr)]
#[display(style = "lowercase")]
enum To {
    Nu,
}

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "build")]
    Build {
        /// target config file
        #[structopt(parse(from_os_str), short, long = "file")]
        fpath: PathBuf,

        /// target config file [possible values: nu]
        #[structopt(short, long, default_value = "nu")]
        to: To,

        /// [possible values: midi, json]
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
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Build { fpath, to, replace } => {
            let content = std::fs::read_to_string(fpath).expect("Unable to read file");
            let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
            let config = Config::from(raw);

            let sh = match to {
                To::Nu => NuConfig::from(config),
            };

            if replace {
                sh.write();
            } else {
                sh.print();
            }
        }
        Opt::Install { name, fpath } => {
            let content = std::fs::read_to_string(fpath).expect("Unable to read file");
            let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
            let config = Config::from(raw);
            if let Some(command) = config.dependencies.get(&name) {
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
                        println!("Success to install {}", command);
                    }
                }
            } else {
                eprintln!("Not found {}", name)
            }
        }
    }
}
