//! Shell information detection module

use crate::{context::SystemContext, DetectionResult, Module, ModuleInfo, ModuleKind};
use std::fmt;

/// Shell detection module
#[derive(Debug)]
pub struct ShellModule;

/// Shell information
#[derive(Debug, Clone)]
pub struct ShellInfo {
    pub name: String,
    pub version: Option<String>,
}

impl fmt::Display for ShellInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ref version) = self.version {
            write!(f, " {version}")?;
        }
        Ok(())
    }
}

impl Module for ShellModule {
    fn detect(&self, ctx: &dyn SystemContext) -> DetectionResult<ModuleInfo> {
        detect_shell(ctx).map(ModuleInfo::Shell)
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::Shell
    }
}

#[cfg(unix)]
fn detect_shell(ctx: &dyn SystemContext) -> DetectionResult<ShellInfo> {
    use std::path::Path;

    // Get shell from SHELL environment variable
    let shell_path = ctx.get_env("SHELL").unwrap_or_else(|| String::from("/bin/sh"));

    // Extract shell name from path
    let name = Path::new(&shell_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("sh")
        .to_string();

    // Try to get version for common shells
    let version = match name.as_str() {
        "bash" => get_command_version(ctx, "bash", &["--version"]),
        "zsh" => get_command_version(ctx, "zsh", &["--version"]),
        "fish" => get_command_version(ctx, "fish", &["--version"]),
        "ksh" => get_command_version(ctx, "ksh", &["--version"]),
        "tcsh" => get_command_version(ctx, "tcsh", &["--version"]),
        _ => None,
    };

    DetectionResult::Detected(ShellInfo { name, version })
}

#[cfg(unix)]
fn get_command_version(ctx: &dyn SystemContext, cmd: &str, args: &[&str]) -> Option<String> {
    let output = ctx.execute_command(cmd, args).ok()?;

    if output.success {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Extract version from first line
        let first_line = stdout.lines().next()?.trim();

        // Try to extract version number from the output
        // Common pattern: "name version X.Y.Z"
        if let Some(version_part) = first_line.split_whitespace().last() {
            // Check if it looks like a version number
            if version_part.chars().next()?.is_ascii_digit() {
                return Some(version_part.to_string());
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn detect_shell(ctx: &dyn SystemContext) -> DetectionResult<ShellInfo> {
    let comspec = ctx.get_env("COMSPEC").unwrap_or_else(|| "cmd.exe".to_string());

    let name = std::path::Path::new(&comspec)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("cmd")
        .to_string();

    DetectionResult::Detected(ShellInfo {
        name,
        version: None,
    })
}

#[cfg(not(any(unix, target_os = "windows")))]
fn detect_shell(_ctx: &dyn SystemContext) -> DetectionResult<ShellInfo> {
    use crate::error::Error;
    DetectionResult::Error(Error::UnsupportedPlatform)
}
