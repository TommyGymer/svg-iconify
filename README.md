# SVG Iconify
[![Rust][action-badge]][action-url]
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]

[action-badge]: https://github.com/TommyGymer/svg-inconify/actions/workflows/rust.yml/badge.svg
[action-url]: https://github.com/TommyGymer/svg-inconify/actions/workflows/rust.yml
[crates-badge]: https://img.shields.io/crates/v/svg-iconify.svg
[crates-url]: https://crates.io/crates/svg-iconify
[docs-badge]: https://docs.rs/svg-iconify/badge.svg
[docs-url]: https://docs.rs/svg-iconify

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
