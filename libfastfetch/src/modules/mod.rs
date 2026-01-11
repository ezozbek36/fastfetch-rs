//! Module system for system information detection
//!
//! This module provides the core trait and enum dispatch system for
//! detecting various system information.

pub mod cpu;
pub mod host;
pub mod kernel;
pub mod memory;
pub mod os;
pub mod shell;
pub mod uptime;

use crate::DetectionResult;
use std::{fmt, str::FromStr};

/// Module trait for all detection modules
pub trait Module: Send + Sync {
    /// Detect information for this module
    ///
    /// Returns:
    /// - `Detected(info)` if detection succeeded
    /// - `Unavailable` if information is unavailable
    /// - `Error(e)` if detection failed with an error
    fn detect(&self) -> DetectionResult<ModuleInfo>;

    /// Get the module kind
    fn kind(&self) -> ModuleKind;

    /// Get the module name
    fn name(&self) -> &'static str {
        self.kind().name()
    }
}

/// Enum representing all available module types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModuleKind {
    Os,
    Host,
    Kernel,
    Uptime,
    Shell,
    Cpu,
    Memory,
}

impl ModuleKind {
    /// Get the display name for this module
    pub const fn name(self) -> &'static str {
        match self {
            Self::Os => "OS",
            Self::Host => "Host",
            Self::Kernel => "Kernel",
            Self::Uptime => "Uptime",
            Self::Shell => "Shell",
            Self::Cpu => "CPU",
            Self::Memory => "Memory",
        }
    }

    /// Get all available module kinds
    pub const fn all() -> &'static [Self] {
        &[
            Self::Os,
            Self::Host,
            Self::Kernel,
            Self::Uptime,
            Self::Shell,
            Self::Cpu,
            Self::Memory,
        ]
    }
}

impl FromStr for ModuleKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "os" => Ok(Self::Os),
            "host" => Ok(Self::Host),
            "kernel" => Ok(Self::Kernel),
            "uptime" => Ok(Self::Uptime),
            "shell" => Ok(Self::Shell),
            "cpu" => Ok(Self::Cpu),
            "memory" => Ok(Self::Memory),
            _ => Err(format!("Unknown module: {s}")),
        }
    }
}

impl fmt::Display for ModuleKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Information returned by a module
#[derive(Debug, Clone)]
pub enum ModuleInfo {
    Os(os::OsInfo),
    Host(host::HostInfo),
    Kernel(kernel::KernelInfo),
    Uptime(uptime::UptimeInfo),
    Shell(shell::ShellInfo),
    Cpu(cpu::CpuInfo),
    Memory(memory::MemoryInfo),
}

impl fmt::Display for ModuleInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Os(info) => write!(f, "{info}"),
            Self::Host(info) => write!(f, "{info}"),
            Self::Kernel(info) => write!(f, "{info}"),
            Self::Uptime(info) => write!(f, "{info}"),
            Self::Shell(info) => write!(f, "{info}"),
            Self::Cpu(info) => write!(f, "{info}"),
            Self::Memory(info) => write!(f, "{info}"),
        }
    }
}

/// Create a module instance for the given kind
pub fn create_module(kind: ModuleKind) -> Box<dyn Module> {
    match kind {
        ModuleKind::Os => Box::new(os::OsModule),
        ModuleKind::Host => Box::new(host::HostModule),
        ModuleKind::Kernel => Box::new(kernel::KernelModule),
        ModuleKind::Uptime => Box::new(uptime::UptimeModule),
        ModuleKind::Shell => Box::new(shell::ShellModule),
        ModuleKind::Cpu => Box::new(cpu::CpuModule),
        ModuleKind::Memory => Box::new(memory::MemoryModule),
    }
}
