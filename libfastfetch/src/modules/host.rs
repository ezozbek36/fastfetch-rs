//! Host information detection module

use crate::{Module, ModuleInfo, ModuleKind, Result};
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
    fn detect(&self) -> Result<ModuleInfo> {
        let info = detect_host()?;
        Ok(info.map(ModuleInfo::Host))
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Host
    }
}

#[cfg(unix)]
fn detect_host() -> Result<HostInfo> {
    use std::ffi::CStr;

    let mut buf = [0u8; 256];
    let result = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

    if result == 0 {
        let hostname = unsafe { CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }
            .to_string_lossy()
            .to_string();

        Ok(Some(HostInfo { hostname }))
    } else {
        Ok(None)
    }
}

#[cfg(target_os = "windows")]
fn detect_host() -> Result<HostInfo> {
    use std::env;

    let hostname = env::var("COMPUTERNAME")
        .or_else(|_| env::var("HOSTNAME"))
        .unwrap_or_else(|_| "Unknown".to_string());

    Ok(Some(HostInfo { hostname }))
}

#[cfg(not(any(unix, target_os = "windows")))]
fn detect_host() -> Result<HostInfo> {
    use crate::error::Error;
    Err(Error::UnsupportedPlatform.into())
}
