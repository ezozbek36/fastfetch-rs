//! Terminal output and formatting layer.
//!
//! Provides a small vertical slice for formatting module results, with
//! optional logo rendering and values-only output.

pub mod color;

use crate::{ModuleKind, logo::Logo};
pub use color::{Color, Style, StyledString};

/// Render-ready module entry containing formatted value or error text.
#[derive(Debug, Clone)]
pub struct RenderedModule {
    pub kind: ModuleKind,
    pub value: Option<String>,
    pub error: Option<String>,
}

impl RenderedModule {
    pub fn value(kind: ModuleKind, value: String) -> Self {
        Self {
            kind,
            value: Some(value),
            error: None,
        }
    }

    pub fn unavailable(kind: ModuleKind) -> Self {
        Self {
            kind,
            value: None,
            error: None,
        }
    }

    pub fn error(kind: ModuleKind, error: String) -> Self {
        Self {
            kind,
            value: None,
            error: Some(error),
        }
    }
}

/// Formats output for the terminal, optionally combining a logo with module lines.
#[derive(Debug, Clone)]
pub struct OutputFormatter {
    values_only: bool,
    logo: Option<Logo>,
}

impl OutputFormatter {
    pub fn new(values_only: bool, logo: Option<Logo>) -> Self {
        Self { values_only, logo }
    }

    /// Format results into a single string ready for printing.
    pub fn render(&self, modules: &[RenderedModule]) -> String {
        let mut lines = Vec::new();

        if !self.values_only {
            lines.push("fastfetch-rs".to_string());
            lines.push(String::new());
        }

        let label_width = modules
            .iter()
            .map(|m| m.kind.name().len())
            .max()
            .unwrap_or(0);

        for module in modules {
            match (&module.value, &module.error) {
                (Some(value), _) if self.values_only => {
                    lines.push(value.clone());
                }
                (Some(value), _) => {
                    lines.push(format!("{:<label_width$}: {value}", module.kind.name()));
                }
                (None, Some(err)) if !self.values_only => {
                    lines.push(format!(
                        "{:<label_width$}: Error - {err}",
                        module.kind.name()
                    ));
                }
                (None, None) if !self.values_only => {
                    lines.push(format!(
                        "{:<label_width$}: Not available",
                        module.kind.name()
                    ));
                }
                _ => {}
            }
        }

        match &self.logo {
            Some(logo) => self.merge_with_logo(lines, logo),
            None => lines.join("\n"),
        }
    }

    fn merge_with_logo(&self, lines: Vec<String>, logo: &Logo) -> String {
        let logo_lines = logo.lines();
        let total_lines = lines.len().max(logo_lines.len());
        let mut rendered = Vec::with_capacity(total_lines);
        let spacer = "  ";

        for idx in 0..total_lines {
            let logo_line = logo_lines.get(idx).map(String::as_str).unwrap_or("");
            let content_line = lines.get(idx).map(String::as_str).unwrap_or("");

            // Calculate visible width (excluding ANSI codes)
            let visible_width = logo_line
                .chars()
                .filter(|&c| c != '\x1b')
                .collect::<String>()
                .replace("[0m", "")
                .replace("[1m", "")
                .replace(
                    |c: char| c.is_ascii_digit() || c == '[' || c == ';' || c == 'm',
                    "",
                )
                .len();

            let padding = logo.width().saturating_sub(visible_width);
            rendered.push(format!("{logo_line}{:padding$}{spacer}{content_line}", ""));
        }

        rendered.join("\n")
    }
}
