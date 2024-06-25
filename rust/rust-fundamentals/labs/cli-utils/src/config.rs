//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

/// Implementation of the Logging struct.
/// # Examples:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let mut config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// config.disable();
/// config.enable();
/// config.set_level(LogLevel::Debug);
/// config.set_destination(LogOutput::Stderr);
/// ```
impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }
}
