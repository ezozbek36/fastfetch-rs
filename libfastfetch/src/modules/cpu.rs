//! CPU information detection module

use crate::{context::SystemContext, DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;
use std::path::Path;

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
    fn detect(&self, ctx: &dyn SystemContext) -> DetectionResult<ModuleInfo> {
        detect_cpu(ctx).map(ModuleInfo::Cpu)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Cpu
    }
}

#[cfg(target_os = "linux")]
fn detect_cpu(ctx: &dyn SystemContext) -> DetectionResult<CpuInfo> {
    let cpuinfo = match ctx.read_file(Path::new("/proc/cpuinfo")) {
        Ok(content) => content,
        Err(err) => return DetectionResult::Error(err.into()),
    };

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

    DetectionResult::Detected(CpuInfo { model, cores })
}

#[cfg(target_os = "macos")]
fn detect_cpu(ctx: &dyn SystemContext) -> DetectionResult<CpuInfo> {
    let model_output = match ctx.execute_command("sysctl", &["-n", "machdep.cpu.brand_string"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let model = if model_output.success {
        String::from_utf8_lossy(&model_output.stdout)
            .trim()
            .to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cores_output = match ctx.execute_command("sysctl", &["-n", "hw.physicalcpu"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let cores = if cores_output.success {
        String::from_utf8_lossy(&cores_output.stdout)
            .trim()
            .parse()
            .ok()
    } else {
        None
    };

    DetectionResult::Detected(CpuInfo { model, cores })
}

#[cfg(target_os = "windows")]
fn detect_cpu(ctx: &dyn SystemContext) -> DetectionResult<CpuInfo> {
    let model = ctx
        .get_env("PROCESSOR_IDENTIFIER")
        .unwrap_or_else(|| "Unknown CPU".to_string());

    let cores = ctx.get_env("NUMBER_OF_PROCESSORS").and_then(|s| s.parse().ok());

    DetectionResult::Detected(CpuInfo { model, cores })
}

#[cfg(target_os = "freebsd")]
fn detect_cpu(ctx: &dyn SystemContext) -> DetectionResult<CpuInfo> {
    let model_output = match ctx.execute_command("sysctl", &["-n", "hw.model"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let model = if model_output.success {
        String::from_utf8_lossy(&model_output.stdout)
            .trim()
            .to_string()
    } else {
        "Unknown CPU".to_string()
    };

    let cores_output = match ctx.execute_command("sysctl", &["-n", "hw.ncpu"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let cores = if cores_output.success {
        String::from_utf8_lossy(&cores_output.stdout)
            .trim()
            .parse()
            .ok()
    } else {
        None
    };

    DetectionResult::Detected(CpuInfo { model, cores })
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_cpu(_ctx: &dyn SystemContext) -> DetectionResult<CpuInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
