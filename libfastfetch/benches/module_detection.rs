//! Benchmarks for module detection performance
//!
//! Run with: `cargo bench`
//! View results in: target/criterion/report/index.html

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use libfastfetch::{
    modules::{create_module, ModuleKind},
    Config, RealSystemContext,
};

/// Benchmark individual module detection
fn bench_individual_modules(c: &mut Criterion) {
    let mut group = c.benchmark_group("individual_modules");
    let ctx = RealSystemContext;

    for kind in ModuleKind::all() {
        group.bench_with_input(BenchmarkId::from_parameter(kind), kind, |b, &kind| {
            b.iter(|| {
                let module = create_module(kind);
                black_box(module.detect(&ctx))
            });
        });
    }

    group.finish();
}

/// Benchmark parallel vs sequential module execution
fn bench_parallel_vs_sequential(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_vs_sequential");

    // Sequential execution
    group.bench_function("sequential", |b| {
        let config = Config::builder()
            .parallel(false)
            .with_modules(ModuleKind::all().to_vec())
            .build()
            .config;
        let app = libfastfetch::Application::new(config);

        b.iter(|| black_box(app.run()));
    });

    // Parallel execution
    group.bench_function("parallel", |b| {
        let config = Config::builder()
            .parallel(true)
            .with_modules(ModuleKind::all().to_vec())
            .build()
            .config;
        let app = libfastfetch::Application::new(config);

        b.iter(|| black_box(app.run()));
    });

    group.finish();
}

/// Benchmark different numbers of modules
fn bench_module_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("module_scaling");

    let module_counts = [1, 2, 4, 7]; // 7 is all available modules

    for &count in &module_counts {
        let modules: Vec<ModuleKind> = ModuleKind::all().iter().take(count).copied().collect();

        // Sequential
        group.bench_with_input(
            BenchmarkId::new("sequential", count),
            &modules,
            |b, modules| {
                let config = Config::builder()
                    .parallel(false)
                    .with_modules(modules.clone())
                    .build()
                    .config;
                let app = libfastfetch::Application::new(config);

                b.iter(|| black_box(app.run()));
            },
        );

        // Parallel
        group.bench_with_input(
            BenchmarkId::new("parallel", count),
            &modules,
            |b, modules| {
                let config = Config::builder()
                    .parallel(true)
                    .with_modules(modules.clone())
                    .build()
                    .config;
                let app = libfastfetch::Application::new(config);

                b.iter(|| black_box(app.run()));
            },
        );
    }

    group.finish();
}

/// Benchmark full application run
fn bench_full_app(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_application");

    group.bench_function("default_config", |b| {
        let config = Config::builder().build().config;
        let app = libfastfetch::Application::new(config);

        b.iter(|| black_box(app.run()));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_individual_modules,
    bench_parallel_vs_sequential,
    bench_module_scaling,
    bench_full_app,
);
criterion_main!(benches);

