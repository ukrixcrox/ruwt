//! helps with interacting with the config at $HOME/.config/ruwt/config.toml and below
use std::path::PathBuf;

//TODO: implement

/// struct to help with config
pub struct Config {
    pub path: PathBuf,
}

impl Config {
    /// creates a new Config object (constructor?)
    pub fn new() {
        todo!("return custom error and implement")
    }
}
