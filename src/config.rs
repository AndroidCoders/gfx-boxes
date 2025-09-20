use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    #[cfg(feature = "debug-video-to-png")]
    pub debug: DebugConfig,
    pub objects: Vec<ObjectConfig>,
}

#[derive(Deserialize)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "debug-video-to-png")]
#[derive(Deserialize)]
pub struct DebugConfig {
    pub max_captured_frames: usize,
}

#[derive(Deserialize, Clone)]
pub struct ObjectConfig {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub vx: i32,
    pub vy: i32,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
