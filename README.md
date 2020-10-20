# Rusty FFmpeg

[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master)](https://github.com/CCExtractor/rusty_ffmpeg/actions)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg.svg)](https://crates.io/crates/rusty_ffmpeg)
[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg)](https://docs.rs/rusty_ffmpeg)

Cross platform FFI bindings for FFmpeg inner libraries. This is a crate that:
1. Generates Rust binding for FFmpeg libraries which can be used directly.
2. Emits specific cargo metadata for linking FFmpeg libraries.

## Building

### Generate and build the bindings:  

Library linking is static by default, setting environment variable `FFMPEG_DYNAMIC_LINKING` will make rusty_ffmpeg link dynamic libraries.

#### Linux & MacOS

1. Start to prepare FFmpeg libraries:
    + If you have a pre-built ffmpeg, set `FFMPEG_PKG_CONFIG_PATH` to the path which points to `*.pc` files in the build result(e.g. `FFMPEG_PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build`) then this crate will use the pre-built FFmpeg libraries.  
    + If no `FFMPEG_PKG_CONFIG_PATH` is set, this crate will first check if there are `libav*-dev` installed in this system.
        + If the libraries exists, this crate will use them.
        + If not, it will git clone the FFmpeg from <https://github.com/ffmpeg/ffmpeg> and then configure and compile it for you.
2. After the FFmpeg libraries are ready, the build script will take advantage of the package-config(`*.pc`) files to:  
    1. Probe paths of the header files for binding generation and generate the binding.
    2. Probe library dependencies as project dependencies to ensure this project can be built successfully.

So there are three ways for developers to provide FFmpeg libraries for this crate to generate bindings:  

1. Provide self compiled FFmpeg by setting `FFMPEG_PKG_CONFIG_PATH`
2. Install FFmpeg libraries via system package manager.(Make sure they can be found by pkg-config)
3. Doesn't provide FFmpeg, waiting for this crate cloning and building a FFmpeg with some default configuration from scratch.

### Windows

1. Install vcpkg.
2. Install FFmpeg using it with specific triplet according to your building target and rustflags. (Check [here](https://github.com/ldm0/rusty_ffmpeg/blob/bf4ee3c5c826443426d3f5c1ac6417b43fc88429/.github/workflows/ci.yml#L325)).
3. build with `VCPKG_ROOT` set to the vcpkg path, and `VCPKG_DEFAULT_TRIPLET` set to the triplet you used.

## Testing

You can use `cargo test` to test the generated bindings. Want to see it works? There is a small example for you. Run `cargo run --example slice`.
