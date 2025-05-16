# Rusty FFmpeg

[![Doc](https://docs.rs/rusty_ffmpeg/badge.svg?style=flat-square)](https://docs.rs/rusty_ffmpeg)
[![Crates.io](https://img.shields.io/crates/v/rusty_ffmpeg)](https://crates.io/crates/rusty_ffmpeg)
[![Downloads](https://img.shields.io/crates/d/rusty_ffmpeg)](https://lib.rs/crates/rusty_ffmpeg)
[![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master&style=flat-square)](https://github.com/CCExtractor/rusty_ffmpeg/actions)

Cross platform FFI bindings for FFmpeg internal libraries. This is a crate that:

1. Links FFmpeg libraries for you.
2. Generates Rust binding for FFmpeg libraries.

## Getting started

This create requires configuration before you can start using it. To quickly get started, follow the steps below.

For more advanced usage, consult the [reference section](#reference) below.

Before beginning to build FFmpeg, you should install LLVM first. This will enable Rusty FFmpeg to generate Rust bindings from the C header files. The `bindgen` documentation has [good instructions](https://rust-lang.github.io/rust-bindgen/requirements.html) on how to install prebuilt LLVM binaries for your platform. You can also build LLVM from source, such as using vcpkg, but this can take hours on all but the most well-equipped computers.

### Linux, macOS..(*nix)

If you have FFmpeg installed with your system package manager, import `rusty_ffmpeg` with feature `link_system_ffmpeg` and the feature for your installed FFmpeg version. The build script will use pkg-config to probe for the system-installed FFmpeg.

If you'd like to use an FFmpeg that you built from source, set `FFMPEG_PKG_CONFIG_PATH` to the path of the generated `pkgconfig` directory. The build script will use pkg-config to probe in that directory. You may also use `vcpkg`, as described below: the instructions should be similar to Windows.

### Windows

One of the easiest ways to get started on Windows is to build FFmpeg using [`vcpkg`](https://github.com/microsoft/vcpkg), which provides a [port for FFmpeg](https://vcpkg.io/en/package/ffmpeg).

#### Classic mode of vcpkg

1. Install vcpkg according to the [instructions](https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-powershell#1---set-up-vcpkg) (the first step of cloning and bootstrapping).
2. Build and install ffmpeg: `vcpkg install ffmpeg --triplet=<value>`, where `triplet` is set to an appropriate value for your project ([see below](#vcpkg-triplets)).
3. Check [documentation of the vcpkg *crate*](https://docs.rs/vcpkg) for the environment variables to set.
4. Import `rusty_ffmpeg` with feature `link_vcpkg_ffmpeg` and the feature corresponding to the FFmpeg version that you built.

You may want to look into [`cargo-vcpkg`](https://crates.io/crates/cargo-vcpkg), which can automate these steps.

#### Manifest mode

When using [manifest mode](https://learn.microsoft.com/en-us/vcpkg/concepts/manifest-mode), you create a `vcpkg.json` file in your repository that declares what vcpkg ports your project requires. vcpkg then builds and installs the packages in a `vcpkg_installed` directory that is local to your repository.

1. Create a `vcpkg.json` file for your project by following instructions in vcpkg's documentation. In addition to FFmpeg, make sure you also install the [`pkgconf`](https://vcpkg.io/en/package/pkgconf) port as a host tool.
2. Build and install your manifest: `vcpkg install --triplet=<value>`, where `triplet` is set to an appropriate value for your project ([see below](#vcpkg-triplets)).
3. Set the `PKG_CONFIG` environment variable to the vcpkg-built `pkgconf.exe` tool within the built `tools/` directory. Set the `PKG_CONFIG_PATH` to the `pkgconfig` directory that was installed within the built `lib/` directory.
4. Import `rusty_ffmpeg` with the feature corresponding to the FFmpeg version that you built.

## Reference

Basic usage of Rusty FFmpeg requires that you:

- Set the feature for the FFmpeg version that you are using (see [Features](#features), below).
- [Instruct the build script](#how-the-build-script-finds-ffmpeg) how to find FFmpeg.

### How the build script finds FFmpeg

There are several ways that FFmpeg can be found and configured using the [environment variables](#environment-variables) and [features](#features). There is no default method for finding FFmpeg: you must choose one or more of these methods to utilize.

Unless the `FFMPEG_DLL_PATH` environment variable is set, static linking will be done.

#### Static linking

When the build script is in static linking mode, it will try to statically link with FFmpeg in the following order:

1. First, if `FFMPEG_PKG_CONFIG_PATH` is set, pkg-config or pkgconf is used to probe for FFmpeg by using the `.pc` files in the given directory. The `PKG_CONFIG` and `FFMPEG_BINDING_PATH` environment variables will also be used if they are set.
2. If `FFMPEG_LIBS_DIR` is set, then FFmpeg libraries from that directory will be used. No probing with pkg-config will take place. You must also set the `FFMPEG_INCLUDE_DIR` or the `FFMPEG_BINDING_PATH` environment variable in this case.
3. If the `link_system_ffmpeg` feature is enabled, then pkg-config will be used to probe for an FFmpeg package installed by your system package manager.
4. If the `link_vcpkg_ffmpeg` feature is enabled, then the [vcpkg](https://docs.rs/vcpkg) crate will be used to probe for FFmpeg.
   - Note that as of this writing, the [published crate does not support](https://github.com/mcgoo/vcpkg-rs/issues/41) using vcpkg in [manifest mode](https://learn.microsoft.com/en-us/vcpkg/concepts/manifest-mode), so you may want to consider setting `PKG_CONFIG` and `FFMPEG_PKG_CONFIG_PATH` to the corresponding file/directories installed by vcpkg in that case.

Both `link_system_ffmpeg` and `link_vcpkg_ffmpeg` features can be enabled simultaneously. If system pkg-config probing fails, then it will fallback to vcpkg probing.

#### Dynamic linking

At this time, the build script locates a dynamically-linked FFmpeg using only one way. Probing using pkg-config or vcpkg is not currently supported.

To dynamically link with FFmpeg, the following environment variables must be set:

- `FFMPEG_DLL_PATH`
- `FFMPEG_INCLUDE_DIR` or `FFMPEG_BINDING_PATH`

At this time, more advanced search methods such as those used for static linking are not supported.

### Environment variables

Several aspects of the build are controlled by environment variables:

| Variable | Purpose |
| -------- | ------- |
| `FFMPEG_BINDING_PATH` | Path to a pre-built `binding.rs` file so as to avoid repeatedly regenerating the same binding file. Include files will be ignored in this case; the variable takes precedence over `FFMPEG_INCLUDE_DIR` or any include files discovered using pkg-config. It can be copied from the `OUT_DIR` of a previous build that generated the bindings. |
| `FFMPEG_DLL_PATH` | Path to the FFmpeg dynamically linked library file. The lib file is expected to be alongside it. |
| `FFMPEG_INCLUDE_DIR` | Path to directory containing FFmpeg include files. Bindings will be generated using bindgen. |
| `FFMPEG_LIBS_DIR` | Path to directory containing FFmpeg library files. |
| `FFMPEG_PKG_CONFIG_PATH` | Path to directory containing `.pc` files to probe with pkg-config. |
| `PKG_CONFIG` | Path to pkg-config executable file, if pkg-config is not in your `PATH`. The pkgconf tool can also be used. |

### Features

This crate has several features which can be configured:

| Feature | Purpose |
| ------- | ------- |
| `ffmpeg5` | Compile with support for FFmpeg `5.*` |
| `ffmpeg6` | Compile with support for FFmpeg `6.*` |
| `ffmpeg7` | Compile with support for FFmpeg `7.*` |
| `link_system_ffmpeg` | Enable probing for FFmpeg installed by your system package manager using pkg-config. |
| `link_vcpkg_ffmpeg` | Enable probing for FFmpeg using the [vcpkg](https://docs.rs/vcpkg) crate. |

None of the features are enabled by default. If no `ffmpeg*` feature is chosen, then support for FFmpeg `4.*` will be compiled.

### vcpkg triplets

The vcpkg [triplet](https://learn.microsoft.com/en-us/vcpkg/concepts/triplets) controls the CPU architecture, operating system, runtime library, etc. It is very important that you choose a triplet that is known to be compatible with your Rust toolchain targets and configuration in order to avoid undefined behavior.

This table provides a quick reference for what vcpkg triplet to choose:

| Rust toolchain target | FFmpeg linking | CRT linking | vcpkg triplet |
| --------------------- | -------------- | ----------- | ------------- |
| `x86_64-pc-windows-msvc` | Static | Static | `x64-windows-static` |
| `x86_64-pc-windows-msvc` | Static | Dynamic | `x64-windows-static-md` |
| `x86_64-pc-windows-msvc` | Dynamic | Dynamic | `x64-windows` |
| `x86_64-unknown-linux-gnu` | Static | Dynamic | `x64-linux` |
| `x86_64-unknown-linux-gnu` | Dynamic | Dynamic | `x64-linux-dynamic` |
| `aarch64-apple-darwin` | Static | Dynamic | `arm64-osx` |
| `aarch64-apple-darwin` | Dynamic | Dynamic | `arm64-osx-dynamic` |

- **Rust toolchain target**: View Rust toolchain targets using `rustup target list`.
- **CRT linking**: The C runtime can be dynamically or statically linked to the code compiled by Rust. The [`crt-static`](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) target feature controls this; the default varies by target. On Windows, macOS, and Linux with glibc, dynamic linking is the Rust default.
- **vcpkg triplet**: The vcpkg triplet that you should use for the given Rust target and CRT linking. Typically set using the [`--triplet`](https://learn.microsoft.com/en-us/vcpkg/commands/common-options#triplet) parameter.

## Attention

FFI is not that easy, especially when you are dealing with a big old C project. Don't get discouraged if you encounter some problems. The CI check already has some typical ffmpeg compilation and use cases for you to check. File an issue if you still have any problems.
