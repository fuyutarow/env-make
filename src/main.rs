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
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// target config file [possible values: nu]
        #[structopt(short, long, default_value = "nu")]
        to: To,

        /// [possible values: midi, json]
        #[structopt(short, long)]
        replace: bool,
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
    }
}
