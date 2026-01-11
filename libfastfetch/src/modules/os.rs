//! OS information detection module

use crate::{DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;

/// OS detection module
#[derive(Debug)]
pub struct OsModule;

/// OS information
#[derive(Debug, Clone)]
pub struct OsInfo {
    pub name: String,
    pub version: Option<String>,
    pub arch: String,
}

impl fmt::Display for OsInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ref version) = self.version {
            write!(f, " {version}")?;
        }
        write!(f, " {}", self.arch)
    }
}

impl Module for OsModule {
    fn detect(&self) -> DetectionResult<ModuleInfo> {
        detect_os().map(ModuleInfo::Os)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Os
    }
}

#[cfg(target_os = "linux")]
fn detect_os() -> DetectionResult<OsInfo> {
    use std::fs;

    // Try to read /etc/os-release
    let os_release = match fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
    {
        Ok(content) => content,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let mut name = String::from("Linux");
    let mut version = None;

    for line in os_release.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            name = value.trim_matches('"').to_string();
        } else if version.is_none()
            && let Some(value) = line.strip_prefix("VERSION=")
        {
            version = Some(value.trim_matches('"').to_string());
        }
    }

    DetectionResult::Detected(OsInfo {
        name,
        version,
        arch: std::env::consts::ARCH.to_string(),
    })
}

#[cfg(target_os = "macos")]
fn detect_os() -> DetectionResult<OsInfo> {
    use std::process::Command;

    let output = match Command::new("sw_vers").arg("-productVersion").output() {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let version = if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .into()
    } else {
        None
    };

    DetectionResult::Detected(OsInfo {
        name: "macOS".to_string(),
        version,
        arch: std::env::consts::ARCH.to_string(),
    })
}

#[cfg(target_os = "windows")]
fn detect_os() -> DetectionResult<OsInfo> {
    DetectionResult::Detected(OsInfo {
        name: "Windows".to_string(),
        version: None,
        arch: std::env::consts::ARCH.to_string(),
    })
}

#[cfg(target_os = "freebsd")]
fn detect_os() -> DetectionResult<OsInfo> {
    use std::process::Command;

    let output = match Command::new("uname").arg("-r").output() {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let version = if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .into()
    } else {
        None
    };

    DetectionResult::Detected(OsInfo {
        name: "FreeBSD".to_string(),
        version,
        arch: std::env::consts::ARCH.to_string(),
    })
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_os() -> DetectionResult<OsInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
