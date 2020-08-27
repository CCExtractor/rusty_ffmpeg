use bindgen::{self, callbacks, CargoCallbacks};
use num_cpus as ncpus;
use once_cell::sync::Lazy;
use pkg_config as pkgconfig;

use std::{collections::HashSet, env, fs, path, process::Command};

/// All the libs that FFmpeg has
static LIBS: Lazy<[&str; 8]> = Lazy::new(|| {
    [
        "avcodec",
        "avdevice",
        "avfilter",
        "avformat",
        "avutil",
        "postproc",
        "swresample",
        "swscale",
    ]
});

/// Whitelist of the headers we want to generate bindings
static HEADERS: Lazy<[&str; 64]> = Lazy::new(|| {
    [
        "libavcodec/avcodec.h",
        "libavcodec/avfft.h",
        "libavcodec/dv_profile.h",
        "libavcodec/vaapi.h",
        "libavcodec/vorbis_parser.h",
        "libavdevice/avdevice.h",
        "libavfilter/avfilter.h",
        "libavfilter/buffersink.h",
        "libavfilter/buffersrc.h",
        "libavformat/avformat.h",
        "libavformat/avio.h",
        "libavutil/adler32.h",
        "libavutil/aes.h",
        "libavutil/audio_fifo.h",
        "libavutil/avstring.h",
        "libavutil/avutil.h",
        "libavutil/base64.h",
        "libavutil/blowfish.h",
        "libavutil/bprint.h",
        "libavutil/buffer.h",
        "libavutil/camellia.h",
        "libavutil/cast5.h",
        "libavutil/channel_layout.h",
        "libavutil/cpu.h",
        "libavutil/crc.h",
        "libavutil/dict.h",
        "libavutil/display.h",
        "libavutil/downmix_info.h",
        "libavutil/error.h",
        "libavutil/eval.h",
        "libavutil/fifo.h",
        "libavutil/file.h",
        "libavutil/frame.h",
        "libavutil/hash.h",
        "libavutil/hmac.h",
        "libavutil/imgutils.h",
        "libavutil/lfg.h",
        "libavutil/log.h",
        "libavutil/macros.h",
        "libavutil/mathematics.h",
        "libavutil/md5.h",
        "libavutil/mem.h",
        "libavutil/motion_vector.h",
        "libavutil/murmur3.h",
        "libavutil/opt.h",
        "libavutil/parseutils.h",
        "libavutil/pixdesc.h",
        "libavutil/pixfmt.h",
        "libavutil/random_seed.h",
        "libavutil/rational.h",
        "libavutil/replaygain.h",
        "libavutil/ripemd.h",
        "libavutil/samplefmt.h",
        "libavutil/sha.h",
        "libavutil/sha512.h",
        "libavutil/stereo3d.h",
        "libavutil/threadmessage.h",
        "libavutil/time.h",
        "libavutil/timecode.h",
        "libavutil/twofish.h",
        "libavutil/xtea.h",
        "libpostproc/postprocess.h",
        "libswresample/swresample.h",
        "libswscale/swscale.h",
    ]
});

static PATH: Lazy<String> = Lazy::new(|| env::var("PATH").unwrap());
static OUT_DIR: Lazy<String> = Lazy::new(|| env::var("OUT_DIR").unwrap());
static FFMPEG_DIR: Lazy<String> = Lazy::new(|| format!("{}/ffmpeg", *OUT_DIR));
static BINDING_FILE_PATH: Lazy<String> = Lazy::new(|| format!("{}/binding.rs", *OUT_DIR));
static NUM_CPUS: Lazy<usize> = Lazy::new(ncpus::get);

/// Filter out all symbols in the HashSet, and for others things it will act
/// exactly the same as `CargoCallback`.
#[derive(Debug)]
struct FilterCargoCallbacks(CargoCallbacks, HashSet<String>);

impl callbacks::ParseCallbacks for FilterCargoCallbacks {
    fn will_parse_macro(&self, _name: &str) -> callbacks::MacroParsingBehavior {
        if self.1.contains(_name) {
            callbacks::MacroParsingBehavior::Ignore
        } else {
            callbacks::MacroParsingBehavior::Default
        }
    }
    fn include_file(&self, _filename: &str) {
        self.0.include_file(_filename);
    }
}

fn probe_system_ffmpeg() -> Result<(), String> {
    match (&*LIBS)
        .iter()
        .map(|name| "lib".to_owned() + name)
        .find(|libname| {
            pkgconfig::Config::new()
                .statik(true)
                // Remove side effect by disable metadata emitting
                .cargo_metadata(false)
                .probe(&libname)
                .is_err()
        }) {
        Some(libname) => Err(format!("{} not found", libname)),
        None => Ok(()),
    }
}

fn clone_and_build_ffmpeg() {
    // Check if FFmpeg is cloned.
    if !path::PathBuf::from(format!("{}/fftools", &*FFMPEG_DIR)).is_dir() {
        Command::new("git")
            .current_dir(&*OUT_DIR)
            .args(["clone", "https://github.com/ffmpeg/ffmpeg", "--depth", "1"].iter())
            .spawn()
            .expect("Failed to clone FFmpeg submodule.")
            .wait()
            .expect("Failed to clone FFmpeg submodule.");
    }

    // All outputs are stored in ./ffmpeg/build/{bin, lib, share, include}
    // If no prebuilt FFmpeg libraries provided, we build a custom FFmpeg.

    // Corresponding to the shell script below:
    // ./configure \
    //     --prefix="$PWD/build" \
    //     --extra-cflags="-I$PWD/build/include" \
    //     --extra-ldflags="-L$PWD/build/lib" \
    //     --bindir="$PWD/build/bin" \
    //     --pkg-config-flags="--static" \
    //     --extra-libs="-lpthread -lm" \
    //     --enable-gpl \
    //     --enable-libass \
    //     --enable-libfdk-aac \
    //     --enable-libfreetype \
    //     --enable-libmp3lame \
    //     --enable-libopus \
    //     --enable-libvorbis \
    //     --enable-libvpx \
    //     --enable-libx264 \
    //     --enable-libx265 \
    //     --enable-nonfree
    Command::new(format!("{}/configure", *FFMPEG_DIR))
        .current_dir(&*FFMPEG_DIR)
        .env("PATH", format!("{}/build/bin:{}", *FFMPEG_DIR, *PATH))
        .env(
            "PKG_CONFIG_PATH",
            format!("{}/build/lib/pkgconfig", *FFMPEG_DIR),
        )
        .args(
            [
                format!(r#"--prefix={}/build"#, *FFMPEG_DIR),
                format!(r#"--extra-cflags=-I{}/build/include"#, *FFMPEG_DIR),
                format!(r#"--extra-ldflags=-L{}/build/lib"#, *FFMPEG_DIR),
                format!(r#"--bindir={}/build/bin"#, *FFMPEG_DIR),
            ]
            .iter(),
        )
        .args(
            [
                "--pkg-config-flags=--static",
                "--extra-libs=-lpthread -lm",
                "--enable-gpl",
                "--enable-libass",
                "--enable-libfdk-aac",
                "--enable-libfreetype",
                "--enable-libmp3lame",
                "--enable-libopus",
                "--enable-libvorbis",
                "--enable-libvpx",
                "--enable-libx264",
                "--enable-libx265",
                "--enable-nonfree",
            ]
            .iter(),
        )
        .spawn()
        .expect("FFmpeg build process: configure failed!")
        .wait()
        .expect("FFmpeg build process: configure failed!");

    Command::new("make")
        .current_dir(&*FFMPEG_DIR)
        .env("PATH", format!("{}/build/bin:{}", *FFMPEG_DIR, *PATH))
        .arg(format!("-j{}", *NUM_CPUS))
        .spawn()
        .expect("FFmpeg build process: make compile failed!")
        .wait()
        .expect("FFmpeg build process: make compile failed!");

    Command::new("make")
        .current_dir(&*FFMPEG_DIR)
        .arg(format!("-j{}", *NUM_CPUS))
        .arg("install")
        .spawn()
        .expect("FFmpeg build process: make install failed!")
        .wait()
        .expect("FFmpeg build process: make install failed!");

    /* Commented because it's not needed, we are not using any specific shell.
    Command::new("hash")
        .current_dir(&*FFMPEG_DIR)
        .arg("-r")
        .spawn()
        .expect("FFmpeg build process: clear hash cache failed!")
        .wait()
        .expect("FFmpeg build process: clear hash cache failed!");
    */
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");

    // If it's a documentation generation from docs.rs, just copy the bindings
    // generated locally to `OUT_DIR`. We do this because the building
    // environment of docs.rs doesn't have an network connection, so we cannot
    // git clone the FFmpeg. And they also have a limitation on crate's size:
    // 10MB, which is not enough to fit in FFmpeg source files. So the only
    // thing we can do is copying the locally generated binding files to the
    // `OUT_DIR`.
    if env::var("DOCS_RS").is_ok() {
        fs::copy("src/binding.rs", &*BINDING_FILE_PATH)
            .expect("Prebuilt binding file failed to be copied.");
        return;
    }

    if env::var("PKG_CONFIG_PATH").is_err() {
        if let Err(msg) = probe_system_ffmpeg() {
            // If no system FFmpeg found, download and build one
            eprintln!("{}! Start to git clone an FFmpeg and build.", msg);
            clone_and_build_ffmpeg();
            env::set_var(
                "PKG_CONFIG_PATH",
                format!("{}/build/lib/pkgconfig", *FFMPEG_DIR),
            );
        }
    }
    // Now we can ensure available FFmpeg libraries.

    // Probe libraries(enable emitting cargo metadata)
    // TODO: if specific library is not enabled, we should not probe it. If we
    // want to implement this, we Should modify probe_system_ffmpeg() too.
    let include_paths = (&*LIBS)
        .iter()
        .map(|name| "lib".to_owned() + name)
        .map(|libname| {
            pkgconfig::Config::new()
                // currently only support building with static libraries.
                .statik(true)
                .cargo_metadata(true)
                .probe(&libname)
                .unwrap_or_else(|_| panic!(format!("{} not found!", libname)))
                .include_paths
        })
        .fold(HashSet::new(), |mut acc, paths| {
            paths.into_iter().for_each(|path| {
                acc.insert(path);
            });
            acc
        });

    // Because the strange `FP_*` in `math.h` https://github.com/rust-lang/rust-bindgen/issues/687
    let filter_callback = FilterCargoCallbacks(
        CargoCallbacks,
        vec![
            "FP_NAN".to_owned(),
            "FP_INFINITE".to_owned(),
            "FP_ZERO".to_owned(),
            "FP_SUBNORMAL".to_owned(),
            "FP_NORMAL".to_owned(),
        ]
        .into_iter()
        .collect(),
    );

    // Bindgen the headers
    (&*HEADERS)
        .iter()
        // map header short path to full path
        .map(|header| {
            include_paths
                .iter()
                .find_map(|path| {
                    let full_path = path.join(header);
                    fs::metadata(&full_path).ok().map(|_| full_path)
                })
                .unwrap()
        })
        .fold(
            bindgen::builder()
                // Add clang path, for `#include` header finding in bindgen process.
                .clang_args(
                    include_paths
                        .iter()
                        .map(|path| "-I".to_owned() + path.to_str().unwrap()),
                )
                .parse_callbacks(Box::new(filter_callback)),
            |builder, header| builder.header(header.to_str().unwrap()),
        )
        .generate()
        .expect("Binding generation failed.")
        // Is it correct to generate binding to one file? :-/
        .write_to_file(&*BINDING_FILE_PATH)
        .expect("Cannot write binding to file!")
}
