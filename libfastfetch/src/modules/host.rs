//! Host information detection module

use crate::{DetectionResult, Module, ModuleInfo, ModuleKind};
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
    fn detect(&self) -> DetectionResult<ModuleInfo> {
        detect_host().map(ModuleInfo::Host)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Host
    }
}

#[cfg(unix)]
fn detect_host() -> DetectionResult<HostInfo> {
    use std::ffi::CStr;

    let mut buf = [0u8; 256];
    let result = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

    if result == 0 {
        let hostname = unsafe { CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }
            .to_string_lossy()
            .to_string();

        DetectionResult::Detected(HostInfo { hostname })
    } else {
        DetectionResult::Unavailable
    }
}

#[cfg(target_os = "windows")]
fn detect_host() -> DetectionResult<HostInfo> {
    use std::env;

    let hostname = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "Unknown".to_string());

    DetectionResult::Detected(HostInfo { hostname })
}

#[cfg(not(any(unix, target_os = "windows")))]
fn detect_host() -> DetectionResult<HostInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
