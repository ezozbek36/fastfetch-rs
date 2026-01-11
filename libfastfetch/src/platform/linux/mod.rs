//! Linux-specific implementations
//!
//! Platform layer for parsing /proc, /sys, and other Linux-specific interfaces

pub mod proc;
pub mod sys;

use std::io;

/// Read a single-line file and trim whitespace
pub fn read_single_line(path: &str) -> io::Result<String> {
    std::fs::read_to_string(path).map(|s| s.trim().to_string())
}

/// Parse /etc/os-release or /usr/lib/os-release
pub fn parse_os_release() -> io::Result<OsRelease> {
    let content = std::fs::read_to_string("/etc/os-release")
        .or_else(|_| std::fs::read_to_string("/usr/lib/os-release"))?;

    let mut os_release = OsRelease::default();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let value = value.trim_matches('"');
            match key {
                "ID" => os_release.id = Some(value.to_string()),
                "NAME" => os_release.name = Some(value.to_string()),
                "PRETTY_NAME" => os_release.pretty_name = Some(value.to_string()),
                "VERSION" => os_release.version = Some(value.to_string()),
                "VERSION_ID" => os_release.version_id = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Ok(os_release)
}

/// Parsed /etc/os-release data
#[derive(Debug, Clone, Default)]
pub struct OsRelease {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pretty_name: Option<String>,
    pub version: Option<String>,
    pub version_id: Option<String>,
}
