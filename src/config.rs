use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub silent_shot_enabled: bool,
    pub silent_shot_lethal_key: String,
    pub silent_shot_weapon_swap_key: String,
    pub silent_shot_lethal_key_delay: u64,
    pub silent_shot_weapon_swap_delay: u64,
}

impl Config {
    pub fn init() -> Result<Config, Error> {
        let config = load_config()?;

        Ok(config)
    }
}

pub fn save_config(config: Config) -> Result<(), Error> {
    let config = toml::to_string(&config)?;

    std::fs::write("config.toml", config)?;

    Ok(())
}

pub fn load_config() -> Result<Config, Error> {
    if !std::path::Path::new("config.toml").exists() {
        let config = Config {
            silent_shot_enabled: false,
            silent_shot_lethal_key: "e".to_string(),
            silent_shot_weapon_swap_key: "1".to_string(),
            silent_shot_lethal_key_delay: 20,
            silent_shot_weapon_swap_delay: 100,
        };

        save_config(config.clone())?;

        return Ok(config);
    }

    let config = std::fs::read_to_string("config.toml")?;

    let config = toml::from_str(&config);

    let config = match config {
        Ok(config) => config,
        Err(_) => Config::default(),
    };

    Ok(config)
}
