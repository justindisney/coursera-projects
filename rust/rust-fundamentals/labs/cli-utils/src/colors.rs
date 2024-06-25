//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

pub fn underline(s: &str) -> String {
    format!("\x1b[4m{}\x1b[0m", s)
}

pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Enum representing the different colors available.
pub enum Color {
    Red,
    Green,
    Blue,
    Bold,
    Underline,
}

/// Struct representing a string with a color.
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String,
}

/// Colorize a string with the given color.
impl ColorString {
    /// Create a new ColorString with the given color and string.
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
            Color::Underline => self.colorized = underline(&self.string),
        };
    }

    /// Reset the colorized string to the original string.
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}
