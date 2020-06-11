# Rusty FFmpeg

![CI](https://github.com/CCExtractor/rusty_ffmpeg/workflows/CI/badge.svg?branch=master)

FFI binding for FFmpeg inner library.

#### Building

Prerequisites are a Linux Machine and a successfully builded FFmpeg on it. Run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build` to build it(where `PKG_CONFIG_PATH` points to `*.pc` files in the build result).

#### Testing

After building, you can use `cargo test` to test the generated bindings. Or you can `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo test` directly without building.

To see it works, you can run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo run --example slice`.
