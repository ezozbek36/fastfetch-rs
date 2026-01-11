//! OS information detection module

use crate::{context::SystemContext, DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;
use std::path::Path;

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
    fn detect(&self, ctx: &dyn SystemContext) -> DetectionResult<ModuleInfo> {
        detect_os(ctx).map(ModuleInfo::Os)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Os
    }
}

#[cfg(target_os = "linux")]
fn detect_os(ctx: &dyn SystemContext) -> DetectionResult<OsInfo> {
    // Try to read /etc/os-release
    let os_release = match ctx
        .read_file(Path::new("/etc/os-release"))
        .or_else(|_| ctx.read_file(Path::new("/usr/lib/os-release")))
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
fn detect_os(ctx: &dyn SystemContext) -> DetectionResult<OsInfo> {
    let output = match ctx.execute_command("sw_vers", &["-productVersion"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let version = if output.success {
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
fn detect_os(_ctx: &dyn SystemContext) -> DetectionResult<OsInfo> {
    DetectionResult::Detected(OsInfo {
        name: "Windows".to_string(),
        version: None,
        arch: std::env::consts::ARCH.to_string(),
    })
}

#[cfg(target_os = "freebsd")]
fn detect_os(ctx: &dyn SystemContext) -> DetectionResult<OsInfo> {
    let output = match ctx.execute_command("uname", &["-r"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let version = if output.success {
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
fn detect_os(_ctx: &dyn SystemContext) -> DetectionResult<OsInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
