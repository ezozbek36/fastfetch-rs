//! Minimal logo rendering engine
//!
//! Converts a simple ASCII logo configuration into renderable lines.

use crate::config::LogoConfig;

/// Renderable logo representation.
#[derive(Debug, Clone)]
pub struct Logo {
    lines: Vec<String>,
    width: usize,
}

impl Logo {
    /// Build a logo from configuration, splitting on newlines and measuring width.
    pub fn from_config(config: &LogoConfig) -> Option<Self> {
        let ascii = config.ascii_art.as_ref()?;
        let lines: Vec<String> = ascii.lines().map(|line| line.to_string()).collect();
        let width = lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        if lines.is_empty() {
            None
        } else {
            Some(Self { lines, width })
        }
    }

    /// Width in characters of the widest line.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Lines to render top-to-bottom.
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
}
