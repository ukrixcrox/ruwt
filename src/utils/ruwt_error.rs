//! custom error type for ruwt

use std::path::PathBuf;

pub enum RuwtError {
    /// a general error that can be used for most things
    ///
    /// param error message
    Error(String),

    /// an error related to all filesystem things
    ///
    /// param error message
    /// param location where the error happend
    FsError(String, PathBuf),

    /// an error for all things related to the config
    ///
    /// param error message
    ConfigError(String),

    /// a fatal error,
    /// WARN: this error should not be handled, the program should simply exit with an appropiate
    /// error code and a error message if this error is used
    ///
    /// param error message
    FatalError(String),
}
