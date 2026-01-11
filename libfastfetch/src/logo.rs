//! Logo rendering engine with distribution detection
//!
//! Provides ASCII art logos for various Linux distributions with color support.

pub mod database;

use crate::config::LogoConfig;
use crate::output::{Color, StyledString};

/// Renderable logo representation.
#[derive(Debug, Clone)]
pub struct Logo {
    lines: Vec<String>,
    width: usize,
    color: Option<Color>,
}

impl Logo {
    /// Build a logo from configuration, splitting on newlines and measuring width.
    pub fn from_config(config: &LogoConfig) -> Option<Self> {
        // If custom ASCII art is provided, use it
        if let Some(ref ascii) = config.ascii_art {
            let lines: Vec<String> = ascii.lines().map(|line| line.to_string()).collect();
            let width = lines
                .iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0);

            if lines.is_empty() {
                None
            } else {
                Some(Self {
                    lines,
                    width,
                    color: None,
                })
            }
        } else {
            // Auto-detect distribution logo
            let logo_def = database::detect_logo();
            let width = logo_def
                .lines
                .iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0);

            Some(Self {
                lines: logo_def.lines.iter().map(|s| s.to_string()).collect(),
                width,
                color: logo_def.color,
            })
        }
    }

    /// Width in characters of the widest line.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Lines to render top-to-bottom, with color applied if available.
    pub fn lines(&self) -> Vec<String> {
        if let Some(color) = self.color {
            self.lines
                .iter()
                .map(|line| StyledString::new(line).fg(color).format())
                .collect()
        } else {
            self.lines.clone()
        }
    }
}
