# Rusty FFmpeg

[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master)](https://github.com/CCExtractor/rusty_ffmpeg/actions)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg.svg)](https://crates.io/crates/rusty_ffmpeg)
[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg)](https://docs.rs/rusty_ffmpeg)

FFI bindings for FFmpeg inner libraries. This is a crate that:
1. Generates Rust binding for FFmpeg libraries which can be used directly.
2. Emits specific cargo metadata for linking FFmpeg libraries.

## Building

### Prerequisites  

1. Linux environment.
2. Rust environment([Install](https://www.rust-lang.org/tools/install)).

### Generate and build the bindings:  

Run `cargo build` to build the bindings:

1. Start to prepare FFmpeg libraries:
    + If you have a pre-built ffmpeg, set `PKG_CONFIG_PATH` to the path which points to `*.pc` files in the build result(e.g. `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build`) then this crate will use the pre-built FFmpeg libraries.  
    + If no `PKG_CONFIG_PATH` is set, this crate will first check if there are `libav*-dev` installed in this system.
        + If the libraries exists, this crate will use them.
        + If not, it will git clone the FFmpeg from <https://github.com/ffmpeg/ffmpeg> and then configure and compile it for you.
2. After the FFmpeg libraries are ready, the build script will take advantage of the package-config(`*.pc`) files to:  
    1. Probe paths of the header files for binding generation and generate the binding.
    2. Probe library dependencies as project dependencies to ensure this project can be built successfully.

So there are three ways for developers to provide FFmpeg libraries for this crate to generate bindings:  

1. Provide self compiled FFmpeg by setting `PKG_CONFIG_PATH`
2. Install FFmpeg libraries via system package manager.(Make sure they can be found by pkg-config)
3. Doesn't provide FFmpeg, waiting for this crate cloning and building a FFmpeg with some default configuration from scratch.

## Testing

You can use `cargo test` to test the generated bindings. If you haven't run `cargo build` and you have pre-built FFmpeg libraries. Set the `PKG_CONFIG_PATH` like this: `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo test` which doesn't need to build the FFmpeg separately.

To see it works, you can run `cargo run --example slice`.
