//! Fastfetch library for system information detection
//!
//! This library provides a modular system for detecting and reporting
//! system information across multiple platforms.

pub mod app;
pub mod config;
pub mod context;
pub mod error;
pub mod logo;
pub mod modules;
pub mod output;
pub mod platform;

pub use app::Application;
pub use config::{Config, ConfigBuilder, LogoConfig};
pub use context::{RealSystemContext, SystemContext};
pub use error::{DetectionResult, Error};
pub use modules::{Module, ModuleInfo, ModuleKind};
pub use output::{OutputFormatter, RenderedModule};
