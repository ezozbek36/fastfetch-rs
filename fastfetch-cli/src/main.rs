use libfastfetch::{ModuleKind, modules::*};
use rayon::prelude::*;

fn main() {
    println!("fastfetch-rs");
    println!();

    // Get all module kinds
    let module_kinds = ModuleKind::all();

    // Execute all modules in parallel and collect results
    let results: Vec<_> = module_kinds
        .par_iter()
        .map(|&kind| {
            let module = create_module(kind);
            (kind, module.detect())
        })
        .collect();

    // Display results in order
    for (kind, result) in results {
        match result {
            Ok(Some(info)) => {
                println!("{}: {info}", kind.name());
            }
            Ok(None) => {
                println!("{}: Not available", kind.name());
            }
            Err(e) => {
                eprintln!("{}: Error - {e}", kind.name());
            }
        }
    }
}
