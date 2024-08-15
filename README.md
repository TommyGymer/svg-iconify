# SVG Iconify
[![Rust](https://github.com/TommyGymer/svg-inconify/actions/workflows/rust.yml/badge.svg)](https://github.com/TommyGymer/svg-inconify/actions/workflows/rust.yml)
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/svg-iconify.svg
[crates-url]: https://crates.io/crates/svg-iconify

A CLI for converting SVG's to PNG of specified size

## Installation
```bash
cargo install svg-iconify
```

## Usage
```bash
svg-iconify /home/tom/Pictures/icon.svg 64 64 /home/tom/Pictures/icon-64x64.png
```

## Dependencies
- [resvg](https://crates.io/crates/resvg) for image handling
- [color-eyre](https://crates.io/crates/color-eyre) for result handling
- [clap](https://crates.io/crates/clap) for CLI arg parsing
