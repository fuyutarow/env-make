use cli::Config;
use cli::NuConfig;
use cli::RawConfig;
use cli::ShConfig;

fn main() {
    let fpath = "examples/env.toml";
    let content = std::fs::read_to_string(fpath).expect("Unable to read file");
    let raw = toml::from_str::<RawConfig>(&content).expect("Failed to parse as toml");
    dbg!(&raw);

    let config = Config::from(raw);
    dbg!(&config);

    let nu = NuConfig::from(config);
    dbg!(&nu);

    nu.print();
}
