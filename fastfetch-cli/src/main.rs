use clap::Parser;
use libfastfetch::{ModuleInfo, ModuleKind, Result, modules::create_module};
use rayon::prelude::*;

/// A fast system information tool written in Rust
#[derive(Parser, Debug)]
#[command(name = "fastfetch-rs")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List of modules to display (comma-separated)
    ///
    /// Available modules: os, host, kernel, cpu, memory
    /// If not specified, all modules will be displayed
    #[arg(short, long, value_delimiter = ',')]
    modules: Option<Vec<String>>,

    /// Disable parallel execution of modules
    #[arg(long)]
    no_parallel: bool,

    /// Show only module values without labels
    #[arg(long)]
    values_only: bool,

    /// List all available modules
    #[arg(long)]
    list_modules: bool,
}

fn main() {
    let args = Args::parse();

    // Handle --list-modules flag
    if args.list_modules {
        println!("Available modules:");
        for kind in ModuleKind::all() {
            println!("  - {} ({})", kind.name().to_lowercase(), kind.name());
        }
        return;
    }

    // Determine which modules to run
    let module_kinds = if let Some(ref module_names) = args.modules {
        // Parse requested modules
        let mut kinds = Vec::new();
        for name in module_names {
            match name.parse::<ModuleKind>() {
                Ok(kind) => kinds.push(kind),
                Err(_) => {
                    eprintln!("Warning: Unknown module '{name}', skipping");
                }
            }
        }
        if kinds.is_empty() {
            eprintln!("Error: No valid modules specified");
            std::process::exit(1);
        }
        kinds
    } else {
        // Use all modules
        ModuleKind::all().to_vec()
    };

    // Print header unless values-only mode
    if !args.values_only {
        println!("fastfetch-rs");
        println!();
    }

    // Execute modules
    let results = execute_modules(&module_kinds, args.no_parallel);

    // Display results
    display_results(results, args.values_only);
}

/// Execute modules either in parallel or sequentially
fn execute_modules(
    module_kinds: &[ModuleKind],
    sequential: bool,
) -> Vec<(ModuleKind, Result<ModuleInfo>)> {
    if sequential {
        module_kinds
            .iter()
            .map(|&kind| {
                let module = create_module(kind);
                (kind, module.detect())
            })
            .collect()
    } else {
        module_kinds
            .par_iter()
            .map(|&kind| {
                let module = create_module(kind);
                (kind, module.detect())
            })
            .collect()
    }
}

/// Display module results
fn display_results(results: Vec<(ModuleKind, Result<ModuleInfo>)>, values_only: bool) {
    for (kind, result) in results {
        match result {
            Ok(Some(info)) => {
                if values_only {
                    println!("{info}");
                } else {
                    println!("{}: {info}", kind.name());
                }
            }
            Ok(None) => {
                if !values_only {
                    println!("{}: Not available", kind.name());
                }
            }
            Err(e) => {
                if !values_only {
                    eprintln!("{}: Error - {e}", kind.name());
                }
            }
        }
    }
}
