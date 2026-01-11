//! Host information detection module

use crate::{context::SystemContext, DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;

/// Host detection module
#[derive(Debug)]
pub struct HostModule;

/// Host information
#[derive(Debug, Clone)]
pub struct HostInfo {
    pub hostname: String,
}

impl fmt::Display for HostInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.hostname)
    }
}

impl Module for HostModule {
    fn detect(&self, ctx: &dyn SystemContext) -> DetectionResult<ModuleInfo> {
        detect_host(ctx).map(ModuleInfo::Host)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Host
    }
}

#[cfg(unix)]
fn detect_host(ctx: &dyn SystemContext) -> DetectionResult<HostInfo> {
    match ctx.get_hostname() {
        Ok(hostname) => DetectionResult::Detected(HostInfo { hostname }),
        Err(_) => DetectionResult::Unavailable,
    }
}

#[cfg(target_os = "windows")]
fn detect_host(ctx: &dyn SystemContext) -> DetectionResult<HostInfo> {
    let hostname = ctx
        .get_env("COMPUTERNAME")
        .or_else(|| ctx.get_env("HOSTNAME"))
        .unwrap_or_else(|| "Unknown".to_string());

    DetectionResult::Detected(HostInfo { hostname })
}

#[cfg(not(any(unix, target_os = "windows")))]
fn detect_host(_ctx: &dyn SystemContext) -> DetectionResult<HostInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
