//! Fastfetch library for system information detection
//!
//! This library provides a modular system for detecting and reporting
//! system information across multiple platforms.

pub mod error;
pub mod modules;
pub mod platform;

pub use error::{Error, Result};
pub use modules::{Module, ModuleInfo, ModuleKind};

/// Re-export anyhow for convenience
pub use anyhow;
