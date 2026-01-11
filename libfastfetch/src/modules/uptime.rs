//! Uptime information detection module

use crate::{Module, ModuleInfo, ModuleKind, Result};
use std::fmt;

/// Uptime detection module
#[derive(Debug)]
pub struct UptimeModule;

/// Uptime information (in seconds)
#[derive(Debug, Clone)]
pub struct UptimeInfo {
    pub seconds: u64,
}

impl UptimeInfo {
    /// Format uptime as human-readable string
    fn format_uptime(&self) -> String {
        let days = self.seconds / 86400;
        let hours = (self.seconds % 86400) / 3600;
        let minutes = (self.seconds % 3600) / 60;

        let mut parts = Vec::new();

        if days > 0 {
            parts.push(format!("{days} day{}", if days == 1 { "" } else { "s" }));
        }
        if hours > 0 {
            parts.push(format!("{hours} hour{}", if hours == 1 { "" } else { "s" }));
        }
        if minutes > 0 || parts.is_empty() {
            parts.push(format!(
                "{minutes} minute{}",
                if minutes == 1 { "" } else { "s" }
            ));
        }

        parts.join(", ")
    }
}

impl fmt::Display for UptimeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_uptime())
    }
}

impl Module for UptimeModule {
    fn detect(&self) -> Result<ModuleInfo> {
        let info = detect_uptime()?;
        Ok(info.map(ModuleInfo::Uptime))
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Uptime
    }
}

#[cfg(target_os = "linux")]
fn detect_uptime() -> Result<UptimeInfo> {
    use std::fs;

    let uptime_str = fs::read_to_string("/proc/uptime")?;

    // /proc/uptime format: "uptime_seconds idle_seconds"
    let uptime_seconds = uptime_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|f| f as u64);

    if let Some(seconds) = uptime_seconds {
        Ok(Some(UptimeInfo { seconds }))
    } else {
        Ok(None)
    }
}

#[cfg(target_os = "macos")]
fn detect_uptime() -> Result<UptimeInfo> {
    use std::process::Command;

    let output = Command::new("sysctl")
        .arg("-n")
        .arg("kern.boottime")
        .output()?;

    if output.status.success() {
        let boottime_str = String::from_utf8_lossy(&output.stdout);

        // kern.boottime format: "{ sec = 1234567890, usec = 0 } ..."
        if let Some(sec_part) = boottime_str.split("sec = ").nth(1) {
            if let Some(sec_str) = sec_part.split(',').next() {
                if let Ok(boot_time) = sec_str.trim().parse::<u64>() {
                    // Get current time
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()?
                        .as_secs();

                    let uptime = now.saturating_sub(boot_time);
                    return Ok(Some(UptimeInfo { seconds: uptime }));
                }
            }
        }
    }

    Ok(None)
}

#[cfg(target_os = "windows")]
fn detect_uptime() -> Result<UptimeInfo> {
    // Windows implementation would require Windows API
    // Simplified version using environment or command
    Ok(None)
}

#[cfg(target_os = "freebsd")]
fn detect_uptime() -> Result<UptimeInfo> {
    use std::process::Command;

    let output = Command::new("sysctl")
        .arg("-n")
        .arg("kern.boottime")
        .output()?;

    if output.status.success() {
        let boottime_str = String::from_utf8_lossy(&output.stdout);

        // Similar parsing as macOS
        if let Some(sec_part) = boottime_str.split("sec = ").nth(1) {
            if let Some(sec_str) = sec_part.split(',').next() {
                if let Ok(boot_time) = sec_str.trim().parse::<u64>() {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()?
                        .as_secs();

                    let uptime = now.saturating_sub(boot_time);
                    return Ok(Some(UptimeInfo { seconds: uptime }));
                }
            }
        }
    }

    Ok(None)
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_uptime() -> Result<UptimeInfo> {
    use crate::error::Error;
    Err(Error::UnsupportedPlatform.into())
}
