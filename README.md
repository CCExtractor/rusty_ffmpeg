# Rusty FFmpeg

[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg?style=flat-square)](https://docs.rs/rusty_ffmpeg)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg)](https://crates.io/crates/rusty_ffmpeg)
[![Downloads](https://img.shields.io/crates/d/rusty_ffmpeg)](https://lib.rs/crates/rusty_ffmpeg)
[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master&style=flat-square)](https://github.com/CCExtractor/rusty_ffmpeg/actions)

Cross platform FFI bindings for FFmpeg inner libraries. This is a crate that:

1. Linking FFmpeg libraries for you.
2. Generates Rust binding for FFmpeg libraries.

## Getting started:

To use this crate, you need to set several environment variables.

### The simplest usage:

#### *nix

Build ffmpeg statically and set `FFMPEG_PKG_CONFIG_PATH` to the path of the generated FFmpeg `pkg-config` files. And you don't need to set other environment variables for static linking.

(Hint: setting `FFMPEG_PKG_CONFIG_PATH` to some placeholder value will leave a `rusty_ffmpeg` probing system library.)

#### Windows

`rusty_ffmpeg` can link FFmpeg using `vcpkg`. Install [`vcpkg`](https://github.com/microsoft/vcpkg), check [documentation of the vcpkg *crate*](https://docs.rs/vcpkg) for the environment variables to set, then it works.

### Fine-grained usage:

You need to set several environment variables for both linking and binding generating procedures.

#### To link: 

1. Dynamic linking with pre-built dylib: Set `FFMPEG_DLL_PATH` to the path of `dll` or `so`. (Windows: Put corresponding `.lib` file next to the `.dll` file.)

2. Static linking with pre-built staticlib: Set `FFMPEG_LIBS_DIR` to the path of the FFmpeg pre-built libs directory.

#### To generate bindings: 

1. Compile-time binding generation([requires the `Clang` dylib](https://github.com/KyleMayes/clang-sys/blob/c9ae24a7a218e73e1eccd320174349eef5a3bd1a/build.rs#L23)): Set `FFMPEG_INCLUDE_DIR` to the path to the header files for binding generation.

2. Use your pre-built binding: Set `FFMPEG_BINDING_PATH` to the pre-built binding file. The pre-built binding is usually copied from the `OUT_DIR` of the compile-time binding generation, by using it you don't need to regenerate the same binding file again and again.

### Linking FFmpeg installed by package manager on (*nix)

You can enable system-wide FFmpeg linking by enabling feature `link_system_ffmpeg`.

## Attention

FFI is not that easy, especially when you are dealing with a big old C project. Don't feel depressed when there are some problems. The CI check already has some typical ffmpeg compilation and use cases for you to check. File an issue if you still have any problem.
