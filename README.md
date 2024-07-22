# Rusty FFmpeg

[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg?style=flat-square)](https://docs.rs/rusty_ffmpeg)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg)](https://crates.io/crates/rusty_ffmpeg)
[![Downloads](https://img.shields.io/crates/d/rusty_ffmpeg)](https://lib.rs/crates/rusty_ffmpeg)
[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master&style=flat-square)](https://github.com/CCExtractor/rusty_ffmpeg/actions)

Cross platform FFI bindings for FFmpeg internal libraries. This is a crate that:

1. Links FFmpeg libraries for you.
2. Generates Rust binding for FFmpeg libraries.

## Getting started:

To use this crate, you need to set several environment variables.

### The simplest usage:

#### Linux, macOS..(*nix)

If you have FFmpeg installed with package manager, import `rusty_ffmpeg` with feature `link_system_ffmpeg`. Then it should work.

If you built FFmpeg from source, set `FFMPEG_PKG_CONFIG_PATH` to the path of the generated FFmpeg `pkg-config` directory. Then it should work.

#### Windows

`rusty_ffmpeg` can link FFmpeg using `vcpkg`:
1. Install [`vcpkg`](https://github.com/microsoft/vcpkg), check [documentation of the vcpkg *crate*](https://docs.rs/vcpkg) for the environment variables to set.
2. Import `rusty_ffmpeg` with feature `link_vcpkg_ffmpeg`, Then it should work.

### Fine-grained usage:

You need to set several environment variables for both the linking and binding generating procedures.

#### To link prebuilt libraries: 

1. Dynamic linking with pre-built dylib: Set `FFMPEG_DLL_PATH` to the path of `dll` or `so` files. (Windows: Put corresponding `.lib` file next to the `.dll` file.)

2. Static linking with pre-built staticlib: Set `FFMPEG_LIBS_DIR` to the path of FFmpeg pre-built libs directory.

#### To generate bindings: 

1. Compile-time binding generation([requires the `Clang` dylib](https://github.com/KyleMayes/clang-sys/blob/c9ae24a7a218e73e1eccd320174349eef5a3bd1a/build.rs#L23)): Set `FFMPEG_INCLUDE_DIR` to the path of the header files for binding generation.

2. Use your prebuilt binding: Set `FFMPEG_BINDING_PATH` to the pre-built binding file. The pre-built binding is usually copied from the `OUT_DIR` of the compile-time binding generation, using it will prevent the need to regenerate the same binding file repeatedly.

### Linking FFmpeg installed by package manager on (*nix)

You can link FFmpeg libraries installed by package manager by enabling feature `link_system_ffmpeg` (which uses pkg-config underneath).

### Linking FFmpeg installed by vcpkg

You can link FFmpeg libraries installed by vcpkg by enabling feature `link_vcpkg_ffmpeg` on Windows, macOS, and Linux.

### Use a specific FFmpeg version

- Do nothing when you are using FFmpeg `4.*`
- Enable `ffmpeg5` feature when you are using FFmpeg `5.*`
- Enable `ffmpeg6` feature when you are using FFmpeg `6.*`
- Enable `ffmpeg7` feature when you are using FFmpeg `7.*`

## Attention

FFI is not that easy, especially when you are dealing with a big old C project. Don't get discouraged if you encounter some problems. The CI check already has some typical ffmpeg compilation and use cases for you to check. File an issue if you still have any problems.
