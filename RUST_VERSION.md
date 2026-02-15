# Rust Version Requirement

This project requires **Rust 1.83 or newer** to compile.

## Current System

Your system has Rust 1.75.0, which is too old for some of the project dependencies.

## How to Update Rust

### Option 1: Install rustup (Recommended)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup update
```

### Option 2: System Package Manager

#### Ubuntu/Debian
```bash
# Add official Rust PPA
sudo add-apt-repository ppa:rust-lang/rust
sudo apt update
sudo apt install rustc cargo
```

#### Check Rust has been built from the tarball
If your Rust installation shows "(built from a source tarball)", you likely installed it via system packages. Consider using rustup for easier updates.

## Verify Installation

```bash
rustc --version  # Should show 1.83 or newer
cargo --version
```

## Build the Project

Once Rust is updated:

```bash
cargo build --release
```

## Why This Version?

The project uses modern Rust dependencies that require newer language features:
- `icu_normalizer_data` requires Rust 1.83+ (used by reqwest for URL handling)
- `indexmap` v2+ requires Rust 1.82+ (used by serde_json and config)
- `native-tls` requires Rust 1.80+ (used by reqwest for HTTPS)

These are all transitive dependencies from commonly-used crates like `reqwest`, `chrono`, and `serde`.
