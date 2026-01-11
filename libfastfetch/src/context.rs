//! System context abstraction for dependency injection and testability
//!
//! This module provides traits that abstract system operations, enabling
//! modules to be tested without real filesystem or system calls.

use std::io;
use std::path::Path;

/// Trait abstracting system operations for testability
///
/// Implementations can provide real system access or mock data for testing.
/// This enables modules to be unit tested without requiring actual system resources.
pub trait SystemContext: Send + Sync {
    /// Read a file to string
    fn read_file(&self, path: &Path) -> io::Result<String>;

    /// Execute a command and return stdout
    fn execute_command(&self, program: &str, args: &[&str]) -> io::Result<CommandOutput>;

    /// Get an environment variable
    fn get_env(&self, key: &str) -> Option<String>;

    /// Get hostname (Unix-specific)
    #[cfg(unix)]
    fn get_hostname(&self) -> io::Result<String>;

    /// Get system name via uname (Unix-specific)
    #[cfg(unix)]
    fn uname(&self) -> io::Result<UtsName>;
}

/// Command execution output
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub success: bool,
}

/// Unix system information from uname
#[cfg(unix)]
#[derive(Debug, Clone)]
pub struct UtsName {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

/// Real system context that performs actual system calls
#[derive(Debug, Clone, Copy, Default)]
pub struct RealSystemContext;

impl SystemContext for RealSystemContext {
    fn read_file(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn execute_command(&self, program: &str, args: &[&str]) -> io::Result<CommandOutput> {
        use std::process::Command;

        let output = Command::new(program).args(args).output()?;

        Ok(CommandOutput {
            stdout: output.stdout,
            stderr: output.stderr,
            success: output.status.success(),
        })
    }

    fn get_env(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    #[cfg(unix)]
    fn get_hostname(&self) -> io::Result<String> {
        use std::ffi::CStr;

        let mut buf = [0u8; 256];
        let result = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };

        if result == 0 {
            let hostname = unsafe { CStr::from_ptr(buf.as_ptr() as *const libc::c_char) }
                .to_string_lossy()
                .to_string();
            Ok(hostname)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(unix)]
    fn uname(&self) -> io::Result<UtsName> {
        use std::ffi::CStr;
        use std::mem;

        let mut utsname: libc::utsname = unsafe { mem::zeroed() };
        let result = unsafe { libc::uname(&mut utsname) };

        if result == 0 {
            Ok(UtsName {
                sysname: unsafe { CStr::from_ptr(utsname.sysname.as_ptr()) }
                    .to_string_lossy()
                    .to_string(),
                nodename: unsafe { CStr::from_ptr(utsname.nodename.as_ptr()) }
                    .to_string_lossy()
                    .to_string(),
                release: unsafe { CStr::from_ptr(utsname.release.as_ptr()) }
                    .to_string_lossy()
                    .to_string(),
                version: unsafe { CStr::from_ptr(utsname.version.as_ptr()) }
                    .to_string_lossy()
                    .to_string(),
                machine: unsafe { CStr::from_ptr(utsname.machine.as_ptr()) }
                    .to_string_lossy()
                    .to_string(),
            })
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock system context for testing
    #[derive(Debug, Clone, Default)]
    pub struct MockSystemContext {
        pub files: std::collections::HashMap<String, String>,
        pub commands: std::collections::HashMap<String, CommandOutput>,
        pub env_vars: std::collections::HashMap<String, String>,
        #[cfg(unix)]
        pub hostname: Option<String>,
        #[cfg(unix)]
        pub uname_result: Option<UtsName>,
    }

    impl SystemContext for MockSystemContext {
        fn read_file(&self, path: &Path) -> io::Result<String> {
            self.files
                .get(path.to_str().unwrap())
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }

        fn execute_command(&self, program: &str, _args: &[&str]) -> io::Result<CommandOutput> {
            self.commands
                .get(program)
                .cloned()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Command not found"))
        }

        fn get_env(&self, key: &str) -> Option<String> {
            self.env_vars.get(key).cloned()
        }

        #[cfg(unix)]
        fn get_hostname(&self) -> io::Result<String> {
            self.hostname
                .clone()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Hostname not set"))
        }

        #[cfg(unix)]
        fn uname(&self) -> io::Result<UtsName> {
            self.uname_result
                .clone()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Uname not set"))
        }
    }

    #[test]
    fn test_mock_context_read_file() {
        let mut ctx = MockSystemContext::default();
        ctx.files.insert(
            "/test/file.txt".to_string(),
            "test content".to_string(),
        );

        let result = ctx.read_file(Path::new("/test/file.txt"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test content");
    }

    #[test]
    fn test_mock_context_env() {
        let mut ctx = MockSystemContext::default();
        ctx.env_vars.insert("TEST_VAR".to_string(), "test_value".to_string());

        assert_eq!(ctx.get_env("TEST_VAR"), Some("test_value".to_string()));
        assert_eq!(ctx.get_env("MISSING"), None);
    }
}
