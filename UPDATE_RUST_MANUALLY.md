# Manual Rust Update Instructions

The automated Rust update encountered issues. Please follow these steps to complete the update manually:

## Step 1: Complete Rustup Installation

Rustup has been partially installed. Complete it by running:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
rustup default stable
```

If this hangs or fails, try:

```bash
# Clean partial installation
rm -rf ~/.rustup

# Reinstall rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts and select option 1 (default installation)
```

## Step 2: Configure Your Shell

Add Rust to your PATH permanently:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Step 3: Verify Installation

```bash
rustc --version  # Should show 1.83 or newer (currently 1.93.1 is latest)
cargo --version
```

## Step 4: Build the Project

Once Rust is updated:

```bash
cd /home/nova/bot_10k
cargo build --release
```

## Troubleshooting

### If you get "command not found" errors:
```bash
source $HOME/.cargo/env
```

### If rustup hangs during download:
The system may have memory or network constraints. Try:
```bash
# Use minimal profile (smaller download)
rustup set profile minimal
rustup default stable
```

### Alternative: Use system Rust (if available)
Check if your system package manager has Rust 1.83+:
```bash
sudo apt update
sudo apt install rustc cargo
rustc --version
```

## Current Status

- ✅ Rustup installed at: `~/.cargo/bin/rustup`
- ⚠️  Rust toolchain installation incomplete
- Target version: Rust 1.93.1 (stable)
- Current system Rust: 1.75.0 (too old)

Once Rust 1.83+ is installed, the project will compile successfully.
