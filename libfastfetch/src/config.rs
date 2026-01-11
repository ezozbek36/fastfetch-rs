//! Configuration and options management for fastfetch-rs
//!
//! A minimal vertical slice that mirrors the upstream architecture by
//! separating configuration from execution. Future work can extend this
//! with preset loading, JSON parsing, and per-module option sets.

use crate::modules::ModuleKind;

const DEFAULT_ASCII_LOGO: &str = r#"  ____          _   _     _    
 |  _ \ __ _ ___| |_| |__ (_)___
 | |_) / _` / __| __| '_ \| / __|
 |  _ < (_| \__ \ |_| | | | \__ \
 |_| \_\__,_|___/\__|_| |_|_|___/"#;

/// Logo configuration placeholder.
#[derive(Debug, Clone, Default)]
pub struct LogoConfig {
    /// Optional ASCII logo to render alongside module output.
    pub ascii_art: Option<String>,
}

/// Resolved configuration used by the application orchestrator.
#[derive(Debug, Clone)]
pub struct Config {
    modules: Vec<ModuleKind>,
    parallel: bool,
    values_only: bool,
    logo: Option<LogoConfig>,
}

impl Config {
    /// Builder entrypoint with sensible defaults (all modules, parallel on, labels on).
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Ordered list of modules to execute.
    pub fn modules(&self) -> &[ModuleKind] {
        &self.modules
    }

    /// Whether to execute modules in parallel.
    pub const fn parallel(&self) -> bool {
        self.parallel
    }

    /// Whether to suppress labels and show only values.
    pub const fn values_only(&self) -> bool {
        self.values_only
    }

    /// Optional logo configuration.
    pub fn logo(&self) -> Option<&LogoConfig> {
        self.logo.as_ref()
    }
}

/// Result of building configuration, including any unknown modules that were skipped.
#[derive(Debug, Clone)]
pub struct BuildOutcome {
    pub config: Config,
    pub unknown_modules: Vec<String>,
}

/// Builder for `Config` that can be fed by CLI flags or future file-based settings.
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    modules: Vec<ModuleKind>,
    explicit_modules: bool,
    parallel: bool,
    values_only: bool,
    logo: Option<LogoConfig>,
    unknown_modules: Vec<String>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            modules: ModuleKind::all().to_vec(),
            explicit_modules: false,
            parallel: true,
            values_only: false,
            logo: Some(LogoConfig {
                ascii_art: Some(DEFAULT_ASCII_LOGO.to_string()),
            }),
            unknown_modules: Vec::new(),
        }
    }
}

impl ConfigBuilder {
    /// Replace module list with an explicit ordered set.
    pub fn with_modules(mut self, modules: Vec<ModuleKind>) -> Self {
        self.modules = modules;
        self.explicit_modules = true;
        self
    }

    /// Parse module names, retaining valid ones and tracking unknown entries.
    pub fn with_module_names<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut parsed = Vec::new();
        for name in names {
            let name = name.into();
            match name.parse::<ModuleKind>() {
                Ok(kind) => parsed.push(kind),
                Err(_) => self.unknown_modules.push(name),
            }
        }

        self.modules = parsed;
        self.explicit_modules = true;

        self
    }

    /// Enable or disable parallel execution.
    pub const fn parallel(mut self, enabled: bool) -> Self {
        self.parallel = enabled;
        self
    }

    /// Toggle values-only output.
    pub const fn values_only(mut self, enabled: bool) -> Self {
        self.values_only = enabled;
        self
    }

    /// Attach a simple ASCII logo to render.
    pub fn with_logo_ascii<T: Into<String>>(mut self, logo: T) -> Self {
        self.logo = Some(LogoConfig {
            ascii_art: Some(logo.into()),
        });
        self
    }

    /// Disable logo rendering entirely.
    pub fn without_logo(mut self) -> Self {
        self.logo = None;
        self
    }

    /// Finalize the configuration and surface any unknown module names.
    pub fn build(self) -> BuildOutcome {
        BuildOutcome {
            config: Config {
                modules: self.modules,
                parallel: self.parallel,
                values_only: self.values_only,
                logo: self.logo,
            },
            unknown_modules: self.unknown_modules,
        }
    }
}
