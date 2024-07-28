//! helps with interacting with the config at $HOME/.config/ruwt/config.toml and below
use dirs::{config_dir, home_dir};
use std::fs::{DirBuilder, File};
use std::{fs, path::PathBuf};

use super::ruwt_error::RuwtError;
use super::ruwt_types::RuwtResult;

/// struct to help with config
pub struct RuwtConfig {
    pub config_file_path: PathBuf,
}

impl RuwtConfig {
    /// creates a new RuwtConfig object (constructor?)
    pub fn new() -> RuwtResult<RuwtConfig> {
        // get the config dir path
        let config_dir = match config_dir() {
            Some(path) => path,
            // if the config dir does not exist, try to create it
            None => create_config_dir(),
        };

        let ruwt_config_dir = config_dir.join("ruwt");
        let ruwt_config_file = ruwt_config_dir.join("config.toml");

        // check if the ruwt config dir already exists as to avoid overwriting it,
        // or unnecessarily creating it again

        if !ruwt_config_dir.exists() {
            // create ruwt config dir
            // because write does not do that
            match fs::DirBuilder::new().create(ruwt_config_dir) {
                Ok(()) => (),
                Err(error) => {
                    let errormsg = format!("could not create .config/ruwt/: {error}");
                    return Err(RuwtError::FatalError(errormsg));
                }
            };

            //INFO: we can get away with 1. checking only if the dir at .config/ruwt/ exists,
            //because if it doesn't, then nothing exists and 2. only not overwriting the config
            //file, because in the config file it says where the templates are, so we only create
            //the default template paths and if they aren't there then an error will be thrown
            match fs::write(ruwt_config_file, "") {
                Ok(()) => (),
                Err(error) => {
                    let errormsg = format!(
                        "could not create the config file at .config/ruwt/config.toml: {error}"
                    );
                    return Err(RuwtError::FsError(errormsg, ruwt_config_file));
                }
            };
        }

        Ok(RuwtConfig {
            config_file_path: ruwt_config_file,
        })
    }

    //TODO: implement

    /// write config file
    pub fn write(&self, content: &str) {
        self.config_file_path;
    }

    /// adds a file to the config dir
    ///
    /// param dirpath path of the dir the file is in relative to ~/.config/ruwt/
    /// param filename name of the file to be created
    /// return full filepath or error
    pub fn add_file(&self, dirpath: PathBuf, filename: &str) -> RuwtResult<PathBuf> {
        let full_dirpath = self.config_file_path.join(dirpath);
        let full_path = full_dirpath.join(filename);

        match DirBuilder::new().recursive(true).create(full_dirpath) {
            Ok(()) => (),
            Err(e) => {
                return Err(RuwtError::FsError(
                    String::from("can't create dir to put file in"),
                    full_path,
                ));
            }
        };

        // create file
        match File::create(filename) {
            Ok(_) => Ok(full_path),
            Err(_) => Err(RuwtError::FsError(
                String::from("can't create file"),
                full_path,
            )),
        }
    }

    /// read config file
    pub fn read(&self) {
        todo!("")
    }
}

/// create a config dir at $HOME
///
/// return PathBuf path to config dir
fn create_config_dir() -> PathBuf {
    let home_dir_path = match home_dir() {
        Some(home_dir) => home_dir,
        // WARN: this would be a fatal error/if there is no $HOME then there is nothing we can do,
        // after all we only go so far before abandoning the user to their on devices
        None => {
            panic!("Fatal: Why is there no $HOME? without one the config can't be created");
        }
    };

    fs::DirBuilder::new()
        .create(&home_dir_path.join(".config"))
        .unwrap_or_else(|e| {
            panic!("Fatal: {e}");
        });

    home_dir_path.join(".config")
}
