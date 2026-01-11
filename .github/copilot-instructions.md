# Copilot Instructions for fastfetch-rs

## Big Picture
- Cargo workspace with two crates:
  - `fastfetch-rs`: Binary crate with main entrypoint in [fastfetch-rs/src/main.rs](fastfetch-rs/src/main.rs)
  - `libfastfetch`: Library crate with core functionality in [libfastfetch/src/lib.rs](libfastfetch/src/lib.rs)
- Currently a minimal scaffold intended to grow into a Rust reimplementation of Fastfetch (system info CLI).
- Upstream reference is vendored as a submodule at [3rd_party/fastfetch](3rd_party/fastfetch). Use it to understand desired behavior, modules, options, and output formats.
- Nix-first workflow: development shell and packaging are defined via [flake.nix](flake.nix), [shell.nix](shell.nix), and [default.nix](default.nix).

## Dev Environment
- Enter the reproducible toolchain with: `nix develop` (provides `rustc`, `cargo`, `clippy`, `rustfmt`, `rust-analyzer`, `cargo-watch`, `nixd`, `alejandra`).
- Formatting and linting: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`.
- Nix formatting: `nix fmt` (flake formatter uses alejandra).

## Build, Run, Package
- Cargo (inside dev shell): `cargo build --release` and `cargo run`.
- Nix package: `nix build` → binary at `./result/bin/fastfetch-rs` (derived from the binary crate name in [fastfetch-rs/Cargo.toml](fastfetch-rs/Cargo.toml)).
- CI mirrors this: see [test.yml](.github/workflows/test.yml) — runs `nix flake check`, `nix build`, and `cargo test` inside `nix develop`.

## Project Conventions
- Rust edition: 2024 (see [Cargo.toml](Cargo.toml) workspace definition). Follow code style/naming in [rust.instructions.md](.github/instructions/rust.instructions.md).
- Release profile is optimized (strip, LTO, `opt-level = "z"`, single codegen unit) — keep binaries lean unless there’s a measured need to relax these.
- Workspace structure: root [Cargo.toml](Cargo.toml) defines the workspace, member crates ([fastfetch-rs](fastfetch-rs/Cargo.toml) and [libfastfetch](libfastfetch/Cargo.toml)) have their own Cargo.toml files.
- System/linker setup in Nix shells: [shell.nix](shell.nix) and [default.nix](default.nix) export `RUST_BACKTRACE`, `LD_LIBRARY_PATH`, and `NIX_LDFLAGS` (libiconv pre-wired). Respect/extend these when introducing native deps.

## Adding Dependencies
- Rust crates: edit the appropriate Cargo.toml in [fastfetch-rs/Cargo.toml](fastfetch-rs/Cargo.toml) or [libfastfetch/Cargo.toml](libfastfetch/Cargo.toml). Prefer small, portable dependencies.
- System libs (for FFI or runtime needs): add to `buildInputs`/`nativeBuildInputs` in both [shell.nix](shell.nix) and [default.nix](default.nix). If a new lib needs runtime linking, extend `LD_LIBRARY_PATH`/`NIX_LDFLAGS` similarly.

## Fastfetch Submodule (Reference)
- Location: [3rd_party/fastfetch](3rd_party/fastfetch) (tracked as a submodule; see [.gitmodules](.gitmodules)).
- Use upstream docs and presets (e.g., [README.md](3rd_party/fastfetch/README.md)) to model feature parity and output conventions.
- Typical submodule ops:
  - Initialize/update: `git submodule update --init --recursive`
  - Refresh to remote branch: `git submodule update --remote 3rd_party/fastfetch`

## Where to Put Code
- Place new Rust modules under [libfastfetch/src/](libfastfetch/src) for core functionality, or [fastfetch-rs/src/](fastfetch-rs/src) for CLI-specific code. Wire them from the binary entrypoint [fastfetch-rs/src/main.rs](fastfetch-rs/src/main.rs).
- Keep OS and hardware interrogation code modular to map cleanly to Fastfetch features as they are ported.

## Debugging & Tests
- Backtraces enabled in dev shell (`RUST_BACKTRACE=full` in [shell.nix](shell.nix)). Run via `cargo run` for quick iteration.
- No tests are present yet; CI runs `cargo test`. Add tests under `tests/` or inline `#[cfg(test)]` modules to integrate automatically.

## Quick References
- Flake outputs: formatter (`nix fmt`), dev shell (`nix develop`), package (`nix build`). See [flake.nix](flake.nix).
- Nix build recipe: [default.nix](default.nix) uses `rustPlatform.buildRustPackage` wired to [Cargo.lock](Cargo.lock).
