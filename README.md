# `viv`

A hacked-together Rust implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway's_Game_of_Life). I hope to have the time to improve it in the future.

Both a TUI and a web UI are provided.

## Stability

The current story for stability is summarized below; however, after 0.0.1, I intend to migrate this crate to use Rust 2018, which will change things quite a bit, as one might imagine.

## Building

### TUI

The TUI supports stable Rust as far back as 1.22 (I believe; I'm having difficulty verifying the compatibility of [`termion`](https://crates.io/crates/termion)). Simply build and run using `cargo run` or whatever method you prefer. ANSI terminals are supported.

### Web UI

The web version uses the excellent [`cargo-web`](https://github.com/koute/cargo-web) and [`stdweb`](https://github.com/koute/stdweb). Only nightly Rust is supported for now. To build, it is recommended to [install `cargo-web`](https://github.com/koute/cargo-web#installation) and run `cargo +nightly web deploy` (or `cargo +nightly web start` during active development). See [the `cargo-web` README](https://github.com/koute/cargo-web#readme) for more information about these commands.

### Notes

Only one of the TUI and web UI may be used at once; if you wish to compile both of them, it is recommended to first deploy the web UI and then run the TUI separately.
