//! custom types used in ruwt

use crate::utils::ruwt_error::RuwtError;

/// custom result using RuwtError
pub type RuwtResult<T> = std::result::Result<T, RuwtError>;
