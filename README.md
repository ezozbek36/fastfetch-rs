# fastfetch-rs

A blazingly fast system information tool written in Rust, inspired by [fastfetch](https://github.com/fastfetch-rs/fastfetch).

## Features

- **⚡ Lightning Fast**: Executes in <50ms with parallel module execution
- **🎨 Beautiful Output**: Colored ASCII art logos for 10+ Linux distributions
- **🔧 Modular Architecture**: Clean separation of concerns with trait-based polymorphism
- **🦀 Pure Rust**: No external dependencies for system detection (uses /proc, /sys, libc)
- **🎯 Cross-Platform**: Supports Linux, macOS, FreeBSD (with limited Windows support)
- **📦 Small Binary**: <2MB stripped release binary

## Modules

Currently implemented modules:

- **OS**: Distribution name, version, and architecture
- **Host**: Hostname
- **Kernel**: Kernel name and version
- **Uptime**: System uptime
- **Shell**: Current shell with version
- **CPU**: Processor model and core count
- **Memory**: Used and total RAM

## Supported Distributions

Logos with auto-detection for:

- Arch Linux
- CachyOS
- Manjaro
- Ubuntu
- Debian
- Fedora
- Gentoo
- openSUSE (Leap/Tumbleweed)
- Generic Linux (fallback)

## Installation

### From Source

```bash
git clone https://github.com/ezozbek36/fastfetch-rs
cd fastfetch-rs
cargo build --release
sudo cp target/release/fastfetch-rs /usr/local/bin/fastfetch-rs
```

## Usage

### Basic Usage

```bash
# Display all modules with logo
fastfetch-rs

# Show specific modules
fastfetch-rs --modules os,kernel,cpu

# Values only (no labels)
fastfetch-rs --values-only

# List available modules
fastfetch-rs --list-modules

# Disable parallel execution
fastfetch-rs --no-parallel
```

### Example Output

```
                   -`                    fastfetch-rs
                  .o+`
                 `ooo/                   OS    : Arch Linux x86_64
                `+oooo:                  Host  : my-computer
               `+oooooo:                 Kernel: Linux 6.1.0-arch1-1
               -+oooooo+:                Uptime: 1 day, 5 hours, 23 minutes
             `/:-:++oooo+:               Shell : zsh 5.9
            `/++++/+++++++:              CPU   : Intel Core i7-9700K (8)
           `/++++++++++++++:             Memory: 8.42 GiB / 15.64 GiB
          `/+++ooooooooooooo/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/
```

## Architecture

### Project Structure

```
fastfetch-rs/
├── fastfetch-rs/          # Binary crate (CLI interface)
│   └── src/
│       └── main.rs         # CLI argument parsing with clap
├── libfastfetch/           # Library crate (core logic)
│   └── src/
│       ├── lib.rs          # Public API
│       ├── app.rs          # Application orchestration
│       ├── config.rs       # Configuration system
│       ├── error.rs        # Error types
│       ├── modules/        # Detection modules
│       │   ├── os.rs
│       │   ├── kernel.rs
│       │   ├── cpu.rs
│       │   ├── memory.rs
│       │   ├── shell.rs
│       │   ├── uptime.rs
│       │   └── host.rs
│       ├── platform/       # Platform-specific code
│       │   └── linux/
│       │       ├── proc.rs  # /proc parsers
│       │       └── sys.rs   # /sys parsers
│       ├── output/         # Output formatting
│       │   ├── formatter.rs
│       │   └── color.rs    # Custom ANSI color codes
│       └── logo/           # Logo system
│           ├── renderer.rs
│           └── database.rs # ASCII art database
└── Cargo.toml              # Workspace configuration
```

### Design Patterns

- **Trait-based Polymorphism**: Each module implements the `Module` trait
- **Enum Dispatch**: Type-safe module information with `ModuleInfo` enum
- **Builder Pattern**: Configuration through `ConfigBuilder`
- **Platform Layer**: Clean separation of OS-specific code

## Performance

Benchmarked on an Intel Core i7-13700H:

- **Execution time**: ~40ms (all modules, parallel)
- **Memory usage**: <5MB
- **Binary size**: 1.8MB (stripped release build)

## Development

### Building

```bash
# Debug build
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

### Adding New Modules

1. Create a new file in `libfastfetch/src/modules/`
2. Implement the `Module` trait
3. Add to `ModuleKind` enum in `modules.rs`
4. Update `create_module()` function

Example:

```rust
use crate::{Module, ModuleInfo, ModuleKind, Result};

#[derive(Debug)]
pub struct MyModule;

impl Module for MyModule {
    fn detect(&self) -> Result<ModuleInfo> {
        // Detection logic
        Ok(Some(ModuleInfo::My(MyInfo { ... })))
    }

    fn kind(&self) -> ModuleKind {
        ModuleKind::My
    }
}
```

### Adding New Logos

Add logo to `libfastfetch/src/logo/database.rs`:

```rust
pub fn my_distro() -> LogoDefinition {
    LogoDefinition {
        lines: &[
            "  ASCII  ",
            "  ART    ",
            "  HERE   ",
        ],
        color: Some(Color::BrightBlue),
    }
}
```

Then update `detect_logo()` to match the distribution ID.

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

- Inspired by [fastfetch](https://github.com/fastfetch-rs/fastfetch)
- ASCII logos adapted from [neofetch](https://github.com/dylanaraps/neofetch)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Roadmap

Future modules to implement:

- [ ] GPU detection (PCI enumeration)
- [ ] Display server (X11/Wayland)
- [ ] Desktop Environment
- [ ] Window Manager
- [ ] Terminal emulator
- [ ] Package manager detection
- [ ] Theme/Icons/Font
- [ ] Network information (Local IP, Public IP)
- [ ] Battery status
- [ ] Disk usage
- [ ] TOML configuration file support

## Author

**Ezozbek Rasulov** - [contact@ezozbek.dev](mailto:contact@ezozbek.dev)

## Links

- [Repository](https://github.com/ezozbek36/fastfetch-rs)
- [Original fastfetch](https://github.com/fastfetch-rs/fastfetch)
