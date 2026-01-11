//! Core application orchestration layer.
//!
//! Executes modules according to configuration and delegates formatting
//! to the output layer. This is the minimal vertical slice that wires
//! configuration → detection → output.

use crate::{
    config::Config,
    context::{RealSystemContext, SystemContext},
    logo::Logo,
    modules::{create_module, ModuleKind},
    output::{OutputFormatter, RenderedModule},
    DetectionResult,
};
use rayon::prelude::*;

/// Orchestrates module execution and output formatting.
#[derive(Debug, Clone)]
pub struct Application {
    config: Config,
}

impl Application {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Run configured modules, optionally in parallel.
    pub fn run(&self) -> Vec<RenderedModule> {
        let ctx = RealSystemContext;
        
        if self.config.parallel() {
            self.config
                .modules()
                .par_iter()
                .map(|&kind| Self::detect_module(kind, &ctx))
                .collect()
        } else {
            self.config
                .modules()
                .iter()
                .copied()
                .map(|kind| Self::detect_module(kind, &ctx))
                .collect()
        }
    }

    /// Render output for a set of module results.
    pub fn render(&self, modules: &[RenderedModule]) -> String {
        let logo = self.config.logo().and_then(Logo::from_config);

        let formatter = OutputFormatter::new(self.config.values_only(), logo);
        formatter.render(modules)
    }

    fn detect_module(kind: ModuleKind, ctx: &dyn SystemContext) -> RenderedModule {
        let module = create_module(kind);
        match module.detect(ctx) {
            DetectionResult::Detected(info) => RenderedModule::value(kind, info.to_string()),
            DetectionResult::Unavailable => RenderedModule::unavailable(kind),
            DetectionResult::Error(err) => RenderedModule::error(kind, err.to_string()),
        }
    }
}
