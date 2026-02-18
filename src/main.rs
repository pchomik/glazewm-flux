#![windows_subsystem = "windows"]

use dirs;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub workspaces: Vec<Workspace>,
    pub keybindings: Vec<Keybinding>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub index: u32,
    pub names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Keybinding {
    pub command: String,
    pub binding: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

fn main() {
    let config_path = dirs::config_dir()
        .map(|p| p.join("GlazeWM-Flux").join("config.toml"))
        .expect("Could not find config directory");

    if !config_path.exists() {
        eprintln!("Config file not found: {:?}", config_path);
        std::process::exit(1);
    }

    let config = Config::from_file(&config_path).expect("Failed to load config");
    println!("Loaded config:");
    println!("{:#?}", config);
}
