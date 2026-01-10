//! CPU information detection module

use crate::{Module, ModuleInfo, ModuleKind, Result};
use std::fmt;

/// CPU detection module
#[derive(Debug)]
pub struct CpuModule;

/// CPU information
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub model: String,
    pub cores: Option<usize>,
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.model)?;
        if let Some(cores) = self.cores {
            write!(f, " ({cores})")?;
        }
        Ok(())
    }
}

impl Module for CpuModule {
    fn detect(&self) -> Result<ModuleInfo> {
        let info = detect_cpu()?;
        Ok(info.map(ModuleInfo::Cpu))
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Cpu
    }
}

#[cfg(target_os = "linux")]
fn detect_cpu() -> Result<CpuInfo> {
    use std::fs;

    let cpuinfo = fs::read_to_string("/proc/cpuinfo")?;

    let mut model = String::from("Unknown CPU");
    let mut cores = None;

    for line in cpuinfo.lines() {
        if let Some(value) = line.strip_prefix("model name") {
            if let Some(name) = value.split(':').nth(1) {
                model = name.trim().to_string();
            }
        } else if let Some(value) = line.strip_prefix("cpu cores")
            && let Some(count) = value.split(':').nth(1)
            && let Ok(num) = count.trim().parse()
        {
            cores = Some(num);
            break;
        }
    }

    Ok(Some(CpuInfo { model, cores }))
}

#[cfg(target_os = "macos")]
fn detect_cpu() -> Result<CpuInfo> {
    use std::process::Command;

    let model_output = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string")
        .output()?;

    let model = if model_output.status.success() {
        String::from_utf8_lossy(&model_output.stdout)
            .trim()
            .to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cores_output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.physicalcpu")
        .output()?;

    let cores = if cores_output.status.success() {
        String::from_utf8_lossy(&cores_output.stdout)
            .trim()
            .parse()
            .ok()
    } else {
        None
    };

    Ok(Some(CpuInfo { model, cores }))
}

#[cfg(target_os = "windows")]
fn detect_cpu() -> Result<CpuInfo> {
    use std::env;

    let model = env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "Unknown CPU".to_string());

    let cores = env::var("NUMBER_OF_PROCESSORS")
        .ok()
        .and_then(|s| s.parse().ok());

    Ok(Some(CpuInfo { model, cores }))
}

#[cfg(target_os = "freebsd")]
fn detect_cpu() -> Result<CpuInfo> {
    use std::process::Command;

    let model_output = Command::new("sysctl").arg("-n").arg("hw.model").output()?;

    let model = if model_output.status.success() {
        String::from_utf8_lossy(&model_output.stdout)
            .trim()
            .to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cores_output = Command::new("sysctl").arg("-n").arg("hw.ncpu").output()?;

    let cores = if cores_output.status.success() {
        String::from_utf8_lossy(&cores_output.stdout)
            .trim()
            .parse()
            .ok()
    } else {
        None
    };

    Ok(Some(CpuInfo { model, cores }))
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_cpu() -> Result<CpuInfo> {
    use crate::error::Error;
    Err(Error::UnsupportedPlatform.into())
}
