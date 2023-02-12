use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BrewConfig {
    version: String,
    homebrew_prefix: String,
    homebrew_cask_opts: Vec<String>,
    homebrew_make_jobs: u16,
    homebrew_ruby: String
}

pub fn load_config() -> Result<BrewConfig, String> {
    match dirs::config_dir() {
        Some(mut path) => {
            path.push("neobrew");
            path.push("config.yml");
            serde_yaml::from_str(std::fs::read_to_string(path))
        },
        None => Err("Failed to find config dir".to_string())
    }
}