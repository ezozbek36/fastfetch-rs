//! OS information detection module

use crate::{Module, ModuleInfo, ModuleKind, Result};
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
    fn detect(&self) -> Result<ModuleInfo> {
        let info = detect_os()?;
        Ok(info.map(ModuleInfo::Os))
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Os
    }
}

#[cfg(target_os = "linux")]
fn detect_os() -> Result<OsInfo> {
    use std::fs;

    // Try to read /etc/os-release
    let os_release = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))?;

    let mut name = String::from("Linux");
    let mut version = None;

    for line in os_release.lines() {
        if let Some(value) = line.strip_prefix("PRETTY_NAME=") {
            name = value.trim_matches('"').to_string();
        } else if version.is_none() && let Some(value) = line.strip_prefix("VERSION=") {
            version = Some(value.trim_matches('"').to_string());
        }
    }

    Ok(Some(OsInfo {
        name,
        version,
        arch: std::env::consts::ARCH.to_string(),
    }))
}

#[cfg(target_os = "macos")]
fn detect_os() -> Result<OsInfo> {
    use std::process::Command;

    let output = Command::new("sw_vers").arg("-productVersion").output()?;

    let version = if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .into()
    } else {
        None
    };

    Ok(Some(OsInfo {
        name: "macOS".to_string(),
        version,
        arch: std::env::consts::ARCH.to_string(),
    }))
}

#[cfg(target_os = "windows")]
fn detect_os() -> Result<OsInfo> {
    Ok(Some(OsInfo {
        name: "Windows".to_string(),
        version: None,
        arch: std::env::consts::ARCH.to_string(),
    }))
}

#[cfg(target_os = "freebsd")]
fn detect_os() -> Result<OsInfo> {
    use std::process::Command;

    let output = Command::new("uname").arg("-r").output()?;

    let version = if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string()
            .into()
    } else {
        None
    };

    Ok(Some(OsInfo {
        name: "FreeBSD".to_string(),
        version,
        arch: std::env::consts::ARCH.to_string(),
    }))
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_os() -> Result<OsInfo> {
    use crate::error::Error;
    Err(Error::UnsupportedPlatform.into())
}
