![Logo](logo.png)

[![License: GPL3](https://img.shields.io/badge/License-GNU%20GPL-blue)](https://opensource.org/license/gpl-3-0)
[![Crate](https://img.shields.io/crates/v/dir2sh.svg)](https://crates.io/crates/dir2sh)

## What is dir2sh?

This Rust tool simplifies copying a folder to a remote Linux machine by generating shell commands.

Designed for graphical environments (Linux/Windows), it bypasses the need for manual `scp`/`sftp`/`git` workflows by copying terminal commands directly to your clipboard.  

## Usage

1. Navigate to the folder you want to copy.  
2. Run `dir2sh` (no parameters or configuration needed).  
3. Paste the generated shell commands on the remote machine.  

**Example clipboard output:**
```bash
set +o history
mkdir -p "hello_world"
printf '%s' 'W3BhY2thZ2VdCm5hbWUgPSAiaGVsbG9fd29ybGQiCnZlcnNpb24gPSAiMC4xLjAiCmVkaXRpb24gPSAiMjAyNCIKCltkZXBlbmRlbmNpZXNdCg==' | base64 -d > "hello_world/Cargo.toml"
mkdir -p "hello_world/src"
printf '%s' 'Zm4gbWFpbigpIHsKICAgIHByaW50bG4hKCJIZWxsbywgd29ybGQhIik7Cn0K' | base64 -d > "hello_world/src/main.rs"
set -o history
```

## Installation

After installing Rust (https://rustup.rs/) you can install this minimalistic tool via:

```bash
cargo install dir2sh
```

## Info

- This tool is for small folders. Pasting a folder with 100 kilobytes takes around six seconds. You should use ```cargo clean``` before copying Rust projects with this tool e.g.
- It won't destroy your ```.bash_history``` because of ```set +o history```.
