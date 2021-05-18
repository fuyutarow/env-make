use cli::BashConfig;
use cli::Config;
use cli::RawConfig;
use cli::ShConfig;

fn main() {
    let fpath = "examples/env.toml";
    let content = std::fs::read_to_string(fpath).expect("Unable to read file");
    let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
    dbg!(&raw);

    let config = Config::from(raw);
    dbg!(&config);

    let bash = BashConfig::from(config);
    dbg!(&bash);
    bash.print();

    let fpath = std::path::Path::new("/tmp/ok.sh").to_path_buf();
    bash.write(fpath);
}
