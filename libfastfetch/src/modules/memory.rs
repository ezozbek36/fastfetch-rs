//! Memory information detection module

use crate::{context::SystemContext, DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;
use std::path::Path;

/// Memory detection module
#[derive(Debug)]
pub struct MemoryModule;

/// Memory information (in bytes)
#[derive(Debug, Clone)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
}

impl MemoryInfo {
    /// Get available memory
    pub const fn available(&self) -> u64 {
        self.total.saturating_sub(self.used)
    }

    /// Format bytes as human-readable string
    fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        format!("{size:.2} {}", UNITS[unit_idx])
    }
}

impl fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} / {}",
            Self::format_bytes(self.used),
            Self::format_bytes(self.total)
        )
    }
}

impl Module for MemoryModule {
    fn detect(&self, ctx: &dyn SystemContext) -> DetectionResult<ModuleInfo> {
        detect_memory(ctx).map(ModuleInfo::Memory)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Memory
    }
}

#[cfg(target_os = "linux")]
fn detect_memory(ctx: &dyn SystemContext) -> DetectionResult<MemoryInfo> {
    let meminfo = match ctx.read_file(Path::new("/proc/meminfo")) {
        Ok(content) => content,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let mut total = 0u64;
    let mut available = 0u64;

    for line in meminfo.lines() {
        if let Some(value) = line.strip_prefix("MemTotal:")
            && let Some(kb_str) = value.split_whitespace().next()
            && let Ok(kb) = kb_str.parse::<u64>()
        {
            total = kb * 1024;
        } else if let Some(value) = line.strip_prefix("MemAvailable:")
            && let Some(kb_str) = value.split_whitespace().next()
            && let Ok(kb) = kb_str.parse::<u64>()
        {
            available = kb * 1024;
        }

        if total > 0 && available > 0 {
            break;
        }
    }

    if total > 0 {
        let used = total.saturating_sub(available);
        DetectionResult::Detected(MemoryInfo { total, used })
    } else {
        DetectionResult::Unavailable
    }
}

#[cfg(target_os = "macos")]
fn detect_memory(ctx: &dyn SystemContext) -> DetectionResult<MemoryInfo> {
    let output = match ctx.execute_command("sysctl", &["-n", "hw.memsize"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let total = if output.success {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .unwrap_or(0)
    } else {
        0
    };

    let vm_output = match ctx.execute_command("vm_stat", &[]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let mut free_pages = 0u64;
    if vm_output.success {
        let vm_stat = String::from_utf8_lossy(&vm_output.stdout);
        for line in vm_stat.lines() {
            if let Some(value) = line.strip_prefix("Pages free:") {
                if let Some(pages_str) = value.trim().split_whitespace().next() {
                    if let Ok(pages) = pages_str.trim_end_matches('.').parse::<u64>() {
                        free_pages = pages;
                        break;
                    }
                }
            }
        }
    }

    if total > 0 {
        const PAGE_SIZE: u64 = 4096;
        let available = free_pages * PAGE_SIZE;
        let used = total.saturating_sub(available);
        DetectionResult::Detected(MemoryInfo { total, used })
    } else {
        DetectionResult::Unavailable
    }
}

#[cfg(target_os = "windows")]
fn detect_memory(_ctx: &dyn SystemContext) -> DetectionResult<MemoryInfo> {
    // Simplified implementation - would need Windows API for accurate info
    DetectionResult::Unavailable
}

#[cfg(target_os = "freebsd")]
fn detect_memory(ctx: &dyn SystemContext) -> DetectionResult<MemoryInfo> {
    let output = match ctx.execute_command("sysctl", &["-n", "hw.physmem"]) {
        Ok(output) => output,
        Err(err) => return DetectionResult::Error(err.into()),
    };

    let total = if output.success {
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse()
            .unwrap_or(0)
    } else {
        0
    };

    if total > 0 {
        // Simplified - just return total, used would need more parsing
        DetectionResult::Detected(MemoryInfo { total, used: 0 })
    } else {
        DetectionResult::Unavailable
    }
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
    target_os = "freebsd"
)))]
fn detect_memory(_ctx: &dyn SystemContext) -> DetectionResult<MemoryInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
