# Rusty FFmpeg

FFI binding for FFmpeg inner library.

Run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build` to build it(where `PKG_CONFIG_PATH` points to `*.pc` files). Then you can use `cargo test` to test the generated bindings.

Or you can `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo test` directly.

To see if it works, you can run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo run --example slice`
