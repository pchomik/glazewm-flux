use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub workspaces: Vec<Workspace>,
    pub keybindings: Vec<Keybinding>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Workspace {
    pub name: String,
    pub names: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
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
