//! Implements an error type for the bowling simulator

use std::fmt;

/// Error type for this Bowling Game
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BowlingError(String);

impl BowlingError {
    pub fn new(msg: &str) -> Self {
        BowlingError(msg.to_string())
    }
}

impl fmt::Display for BowlingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
