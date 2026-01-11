//! Kernel information detection module

use crate::{DetectionResult, Module, ModuleInfo, ModuleKind};
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
    fn detect(&self) -> DetectionResult<ModuleInfo> {
        detect_kernel().map(ModuleInfo::Kernel)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Kernel
    }
}

#[cfg(unix)]
fn detect_kernel() -> DetectionResult<KernelInfo> {
    use std::ffi::CStr;
    use std::mem;

    let mut utsname: libc::utsname = unsafe { mem::zeroed() };
    let result = unsafe { libc::uname(&mut utsname) };

    if result == 0 {
        let name = unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) }
            .to_string_lossy()
            .to_string();

        let version = unsafe { CStr::from_ptr(utsname.release.as_ptr()) }
            .to_string_lossy()
            .to_string();

        DetectionResult::Detected(KernelInfo { name, version })
    } else {
        DetectionResult::Unavailable
    }
}

#[cfg(target_os = "windows")]
fn detect_kernel() -> DetectionResult<KernelInfo> {
    DetectionResult::Detected(KernelInfo {
        name: "Windows NT".to_string(),
        version: "Unknown".to_string(),
    })
}

#[cfg(not(any(unix, target_os = "windows")))]
fn detect_kernel() -> DetectionResult<KernelInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
