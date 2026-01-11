use clap::Parser;
use libfastfetch::{Application, Config, ModuleKind};

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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Handle --list-modules flag
    if args.list_modules {
        println!("Available modules:");
        for kind in ModuleKind::all() {
            println!("  - {} ({})", kind.name().to_lowercase(), kind.name());
        }
        return Ok(());
    }

    let builder: libfastfetch::ConfigBuilder = Config::builder()
        .values_only(args.values_only)
        .parallel(!args.no_parallel);

    let builder = if let Some(ref module_names) = args.modules {
        builder.with_module_names(module_names.clone())
    } else {
        builder
    };

    let outcome = builder.build();

    if let Some(module_names) = args.modules.as_ref() {
        if outcome.unknown_modules.len() == module_names.len() {
            eprintln!("Error: No valid modules specified");
            std::process::exit(1);
        }

        for unknown in &outcome.unknown_modules {
            eprintln!("Warning: Unknown module '{unknown}', skipping");
        }
    }

    let app = Application::new(outcome.config);
    let results = app.run();
    let output = app.render(&results);

    println!("{output}");

    Ok(())
}
