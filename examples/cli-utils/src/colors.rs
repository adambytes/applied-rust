//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

// The purpose of documenting code is to help others understand it.
// 

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}
/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", blue("Blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for bold.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", bold("Bold"));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code to reset the color.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", reset("Reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// An enum to represent the color of a string.
/// # Examples: 
/// ```
/// use cli_utils::colors::Color;
/// let color = Color::Red;
/// ```
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// A struct to represent a string with a color.
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

}
