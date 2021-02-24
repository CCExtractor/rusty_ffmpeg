# Rusty FFmpeg

[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master)](https://github.com/CCExtractor/rusty_ffmpeg/actions)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg.svg)](https://crates.io/crates/rusty_ffmpeg)
[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg)](https://docs.rs/rusty_ffmpeg)

Cross platform FFI bindings for FFmpeg inner libraries. This is a crate that:
1. Generates Rust binding for FFmpeg libraries which can be used directly.
2. Emits specific cargo metadata for linking FFmpeg libraries.

## Building

### Generate and build the bindings:  

1. Set(always) `FFMPEG_INCLUDE_DIR` to the path to the header files for generating bindings.

2. Set `FFMPEG_DLL_PATH` for dynamic linking with `dll` or `so`. (Windows: Put corresponding `.lib` file next to the `.dll` file.)

3. Set `FFMPEG_PKG_CONFIG_PATH` for static linking with `pkg-config` files.

4. Set `FFMPEG_LIBS_DIR` for static linking with static libs.

FFMPEG_INCLUDE_DIR=${HOME}/ffmpeg_build/include FFPEG_LIBS_DIR=${HOME}/ffmpeg_build/lib cargo run --example slice
FFMPEG_INCLUDE_DIR=${HOME}/ffmpeg_build/include FFMPEG_PKG_CONFIG_PATH=${HOME}/ffmpeg_build/lib/pkgconfig cargo run --example slice