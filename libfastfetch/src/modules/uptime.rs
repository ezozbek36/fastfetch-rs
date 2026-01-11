//! Uptime information detection module

use crate::{DetectionResult, Module, ModuleInfo, ModuleKind};
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
    fn detect(&self) -> DetectionResult<ModuleInfo> {
        detect_uptime().map(ModuleInfo::Uptime)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Uptime
    }
}

#[cfg(target_os = "linux")]
fn detect_uptime() -> DetectionResult<UptimeInfo> {
    use std::fs;

    let uptime_str = match fs::read_to_string("/proc/uptime") {
        Ok(content) => content,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    // /proc/uptime format: "uptime_seconds idle_seconds"
    let uptime_seconds = uptime_str
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .map(|f| f as u64);

    if let Some(seconds) = uptime_seconds {
        DetectionResult::Detected(UptimeInfo { seconds })
    } else {
        DetectionResult::Unavailable
    }
}

#[cfg(target_os = "macos")]
fn detect_uptime() -> DetectionResult<UptimeInfo> {
    use std::process::Command;

    let output = match Command::new("sysctl")
        .arg("-n")
        .arg("kern.boottime")
        .output()
    {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    if output.status.success() {
        let boottime_str = String::from_utf8_lossy(&output.stdout);

        // kern.boottime format: "{ sec = 1234567890, usec = 0 } ..."
        if let Some(sec_part) = boottime_str.split("sec = ").nth(1) {
            if let Some(sec_str) = sec_part.split(',').next() {
                if let Ok(boot_time) = sec_str.trim().parse::<u64>() {
                    // Get current time
                    if let Ok(duration) = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                    {
                        let now = duration.as_secs();
                        let uptime = now.saturating_sub(boot_time);
                        return DetectionResult::Detected(UptimeInfo { seconds: uptime });
                    }
                }
            }
        }
    }

    DetectionResult::Unavailable
}

#[cfg(target_os = "windows")]
fn detect_uptime() -> DetectionResult<UptimeInfo> {
    // Windows implementation would require Windows API
    // Simplified version using environment or command
    DetectionResult::Unavailable
}

#[cfg(target_os = "freebsd")]
fn detect_uptime() -> DetectionResult<UptimeInfo> {
    use std::process::Command;

    let output = match Command::new("sysctl")
        .arg("-n")
        .arg("kern.boottime")
        .output()
    {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    if output.status.success() {
        let boottime_str = String::from_utf8_lossy(&output.stdout);

        // Similar parsing as macOS
        if let Some(sec_part) = boottime_str.split("sec = ").nth(1) {
            if let Some(sec_str) = sec_part.split(',').next() {
                if let Ok(boot_time) = sec_str.trim().parse::<u64>() {
                    if let Ok(duration) = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                    {
                        let now = duration.as_secs();
                        let uptime = now.saturating_sub(boot_time);
                        return DetectionResult::Detected(UptimeInfo { seconds: uptime });
                    }
                }
            }
        }
    }

    DetectionResult::Unavailable
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_uptime() -> DetectionResult<UptimeInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
