# Rusty FFmpeg

[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg?style=flat-square)](https://docs.rs/rusty_ffmpeg)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg)](https://crates.io/crates/rusty_ffmpeg)
[![Downloads](https://img.shields.io/crates/d/rusty_ffmpeg)](https://lib.rs/crates/rusty_ffmpeg)
[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master&style=flat-square)](https://github.com/CCExtractor/rusty_ffmpeg/actions)

Cross platform FFI bindings for FFmpeg inner libraries. This is a crate that:

1. Linking FFmpeg libraries for you.
2. Generates Rust binding for FFmpeg libraries.

## Usage

To use this crate, you need to set several environment variables.

### Simplest:

Build ffmpeg staticly and set `FFMPEG_PKG_CONFIG_PATH` to the path of the generated FFmpeg package-config files. And you don't need to set other environment variables for static linking.

### More complex

You need to set environment variable for both linking and generating bindings.

#### To link: 

1. Dynamic linking with prebuilt dylib: Set `FFMPEG_DLL_PATH` to the path of `dll` or `so`. (Windows: Put corresponding `.lib` file next to the `.dll` file.)

2. Static linking with prebuilt staticlib:  or set `FFMPEG_LIB_DIR` to the path of the FFmpeg prebuilt libs directory.

#### To generate bindings: 

1. Compile-time binding generation(requires clang dylib): Set `FFMPEG_INCLUDE_DIR` to the path to the header files for binding generation.

2. Use pre-built binding: Set `FFMPEG_BINDING_PATH` to the prebuilt binding file.

## Attention

FFI is not that easy, especially when you are dealing with a big old C project. Don't feel depressed when there are some problems. The CI check already have some typical use cases, you can check it. File an issue if you still have some problem.
