//! Platform-specific implementations
//!
//! This module provides platform-specific detection implementations.
//! Code is organized by platform to ensure clean separation.

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "freebsd")]
pub mod freebsd;

/// Current operating system type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Linux,
    MacOs,
    Windows,
    FreeBsd,
    Unknown,
}

impl Platform {
    /// Get the current platform
    pub const fn current() -> Self {
        #[cfg(target_os = "linux")]
        return Self::Linux;

        #[cfg(target_os = "macos")]
        return Self::MacOs;

        #[cfg(target_os = "windows")]
        return Self::Windows;

        #[cfg(target_os = "freebsd")]
        return Self::FreeBsd;

        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
            target_os = "freebsd"
        )))]
        return Self::Unknown;
    }

    /// Check if the current platform is supported
    pub const fn is_supported(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}
