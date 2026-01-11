//! Custom ANSI color code support
//!
//! Provides color formatting for terminal output without external dependencies.

use std::fmt;

/// ANSI color code for terminal styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8),
}

impl Color {
    /// Get the ANSI escape code for foreground color
    pub const fn fg_code(self) -> &'static str {
        match self {
            Self::Black => "\x1b[30m",
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Blue => "\x1b[34m",
            Self::Magenta => "\x1b[35m",
            Self::Cyan => "\x1b[36m",
            Self::White => "\x1b[37m",
            Self::BrightBlack => "\x1b[90m",
            Self::BrightRed => "\x1b[91m",
            Self::BrightGreen => "\x1b[92m",
            Self::BrightYellow => "\x1b[93m",
            Self::BrightBlue => "\x1b[94m",
            Self::BrightMagenta => "\x1b[95m",
            Self::BrightCyan => "\x1b[96m",
            Self::BrightWhite => "\x1b[97m",
            Self::Rgb(_, _, _) => "", // Handled separately
        }
    }

    /// Format RGB color as ANSI escape sequence
    pub fn fg_rgb_code(&self) -> Option<String> {
        match self {
            Self::Rgb(r, g, b) => Some(format!("\x1b[38;2;{r};{g};{b}m")),
            _ => None,
        }
    }
}

/// ANSI style modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    Reset,
}

impl Style {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Bold => "\x1b[1m",
            Self::Dim => "\x1b[2m",
            Self::Italic => "\x1b[3m",
            Self::Underline => "\x1b[4m",
            Self::Reset => "\x1b[0m",
        }
    }
}

/// A styled string with color and style information
#[derive(Debug, Clone)]
pub struct StyledString {
    text: String,
    fg_color: Option<Color>,
    style: Option<Style>,
}

impl StyledString {
    /// Create a new unstyled string
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self {
            text: text.into(),
            fg_color: None,
            style: None,
        }
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set style
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    /// Format the string with ANSI codes
    pub fn format(&self) -> String {
        let mut result = String::new();

        // Add style
        if let Some(style) = self.style {
            result.push_str(style.code());
        }

        // Add foreground color
        if let Some(color) = self.fg_color {
            if let Some(rgb_code) = color.fg_rgb_code() {
                result.push_str(&rgb_code);
            } else {
                result.push_str(color.fg_code());
            }
        }

        // Add text
        result.push_str(&self.text);

        // Reset if any styling was applied
        if self.fg_color.is_some() || self.style.is_some() {
            result.push_str(Style::Reset.code());
        }

        result
    }
}

impl fmt::Display for StyledString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Helper functions for common color operations
pub mod helpers {
    use super::{Color, StyledString};

    /// Create a bold colored string
    pub fn bold<S: Into<String>>(text: S, color: Color) -> StyledString {
        StyledString::new(text).fg(color).style(super::Style::Bold)
    }

    /// Create a dim colored string
    pub fn dim<S: Into<String>>(text: S, color: Color) -> StyledString {
        StyledString::new(text).fg(color).style(super::Style::Dim)
    }

    /// Create a colored string
    pub fn colored<S: Into<String>>(text: S, color: Color) -> StyledString {
        StyledString::new(text).fg(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_codes() {
        assert_eq!(Color::Red.fg_code(), "\x1b[31m");
        assert_eq!(Color::BrightGreen.fg_code(), "\x1b[92m");
    }

    #[test]
    fn test_styled_string() {
        let styled = StyledString::new("test").fg(Color::Red);
        assert_eq!(styled.format(), "\x1b[31mtest\x1b[0m");
    }

    #[test]
    fn test_rgb_color() {
        let color = Color::Rgb(255, 128, 0);
        assert_eq!(
            color.fg_rgb_code(),
            Some("\x1b[38;2;255;128;0m".to_string())
        );
    }
}
