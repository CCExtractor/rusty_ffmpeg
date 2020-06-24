# Rusty FFmpeg

[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master)](https://github.com/CCExtractor/rusty_ffmpeg/actions)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg.svg)](https://crates.io/crates/rusty_ffmpeg)
[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg)](https://docs.rs/rusty_ffmpeg)

FFI bindings for FFmpeg inner libraries.

## Building

### Prerequisites  
A Linux Machine with the Nightly Rust toolchain. You can use this [one-liner script](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html) to install nightly Rust:

```
$ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
```
### Generate and build the bindings:  

Run `cargo build` to build the bindings. There are three ways for developer to provide FFmpeg libraries for this crate to generate bindings:  

If you have a pre-built ffmpeg, set `PKG_CONFIG_PATH` to the path which points to `*.pc` files in the build result(e.g. `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build`) then it will use the pre-built FFmpeg libraries.  
If no `PKG_CONFIG_PATH` is set, it will first check if there are `libav*-dev` installed. If the libraries exists, this crate will use them. If not, it will git clone the FFmpeg from <https://github.com/ffmpeg/ffmpeg> and then configure and compile it for you.

After the FFmpeg libraries is ready, the build script will take advantage of the package-config(`*.pc`) files to:  
1. Probe paths of the header files for binding generation and generate the binding.
2. Probe library dependencies as project dependencies to ensure this project can be built successfully.

## Testing

You can use `cargo test` to test the generated bindings. If you haven't run `cargo build` and you have pre-built FFmpeg libraries. Set the `PKG_CONFIG_PATH` like this: `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo test` which doesn't need to build the FFmpeg separately.

To see it works, you can run `cargo run --example slice`.
