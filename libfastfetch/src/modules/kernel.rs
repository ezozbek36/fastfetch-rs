//! Kernel information detection module

use crate::{context::SystemContext, DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;

/// Kernel detection module
#[derive(Debug)]
pub struct KernelModule;

/// Kernel information
#[derive(Debug, Clone)]
pub struct KernelInfo {
    pub name: String,
    pub version: String,
}

impl fmt::Display for KernelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}

impl Module for KernelModule {
    fn detect(&self, ctx: &dyn SystemContext) -> DetectionResult<ModuleInfo> {
        detect_kernel(ctx).map(ModuleInfo::Kernel)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Kernel
    }
}

#[cfg(unix)]
fn detect_kernel(ctx: &dyn SystemContext) -> DetectionResult<KernelInfo> {
    match ctx.uname() {
        Ok(utsname) => DetectionResult::Detected(KernelInfo {
            name: utsname.sysname,
            version: utsname.release,
        }),
        Err(_) => DetectionResult::Unavailable,
    }
}

#[cfg(target_os = "windows")]
fn detect_kernel(_ctx: &dyn SystemContext) -> DetectionResult<KernelInfo> {
    DetectionResult::Detected(KernelInfo {
        name: "Windows NT".to_string(),
        version: "Unknown".to_string(),
    })
}

#[cfg(not(any(unix, target_os = "windows")))]
fn detect_kernel(_ctx: &dyn SystemContext) -> DetectionResult<KernelInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
