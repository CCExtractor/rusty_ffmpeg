# Rusty FFmpeg

[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master)](https://github.com/CCExtractor/rusty_ffmpeg/actions)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg.svg)](https://crates.io/crates/rusty_ffmpeg)
[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg)](https://docs.rs/rusty_ffmpeg)

FFI binding for FFmpeg inner library.

#### Building

1. Prerequisites  
    A Linux Machine with the Nightly Rust toolchain. You can use this [one-liner script](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html) to install nightly Rust:

    ```
    $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
    ```

2. Build the FFmpeg (Skip this step if you already have a built FFmpeg)  
    FFmpeg is a submodule of this repo, you can fetch it by using `git submodule update --init`. Then `cd ffmpeg` and follow the steps of [official installation guide](https://trac.ffmpeg.org/wiki/CompilationGuide) to compile it.
3. Generate and build the bindings:  
    Run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build` to build the binding (Where `PKG_CONFIG_PATH` points to `*.pc` files in the build result). The build script will take advantage of the package-config(`*.pc`) files to probe paths of the header files for binding generation and dependencies as project build dependencies to ensure this project can be built successfully.

#### Testing

After building, you can use `cargo test` to test the generated bindings. Or you can `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo test` directly without building.

To see it works, you can run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo run --example slice`.
