use bindgen::{self, callbacks, Bindings, CargoCallbacks};
use once_cell::sync::Lazy;

use std::{collections::HashSet, env, fs, path};

/// All the libs that FFmpeg has
static LIBS: Lazy<[&str; 8]> = Lazy::new(|| {
    [
        "libavcodec",
        "libavdevice",
        "libavfilter",
        "libavformat",
        "libavutil",
        "libpostproc",
        "libswresample",
        "libswscale",
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

fn generate_bindings(
    include_paths: &HashSet<path::PathBuf>,
    headers: &[&str],
) -> Result<Bindings, ()> {
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
    headers
        .iter()
        // map header short path to full path
        .map(|header| {
            include_paths
                .iter()
                .find_map(|path| {
                    let full_path = path.join(header);
                    // Check if full path valid
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
}

#[cfg(not(target_os = "windows"))]
mod non_windows {
    use super::*;
    use std::process::Command;

    fn try_probe_system_ffmpeg(library_names: &[&str]) -> Result<(), String> {
        match library_names.iter().find(|libname| {
            pkg_config::Config::new()
                // Remove side effect by disable metadata emitting
                .cargo_metadata(false)
                .probe(&libname)
                .is_err()
        }) {
            Some(&libname) => Err(libname.to_string()),
            None => Ok(()),
        }
    }

    fn clone_and_build_ffmpeg(out_dir: &str) {
        let ffmpeg_dir = &format!("{}/ffmpeg", out_dir);

        // Check if FFmpeg is cloned.
        if !path::PathBuf::from(format!("{}/fftools", ffmpeg_dir)).is_dir() {
            Command::new("git")
                .current_dir(out_dir)
                .args(["clone", "https://github.com/ffmpeg/ffmpeg", "--depth", "1"].iter())
                .spawn()
                .expect("Failed to clone FFmpeg submodule.")
                .wait()
                .expect("Failed to clone FFmpeg submodule.");
        }

        let path = &format!("{}/build/bin:{}", ffmpeg_dir, env::var("PATH").unwrap());

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
        Command::new(format!("{}/configure", ffmpeg_dir))
            .current_dir(ffmpeg_dir)
            .env("PATH", path)
            .env(
                "PKG_CONFIG_PATH",
                format!("{}/build/lib/pkgconfig", ffmpeg_dir),
            )
            .args(
                [
                    format!(r#"--prefix={}/build"#, ffmpeg_dir),
                    format!(r#"--extra-cflags=-I{}/build/include"#, ffmpeg_dir),
                    format!(r#"--extra-ldflags=-L{}/build/lib"#, ffmpeg_dir),
                    format!(r#"--bindir={}/build/bin"#, ffmpeg_dir),
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

        let num_cpus = num_cpus::get();

        Command::new("make")
            .current_dir(ffmpeg_dir)
            .env("PATH", path)
            .arg(format!("-j{}", num_cpus))
            .spawn()
            .expect("FFmpeg build process: make compile failed!")
            .wait()
            .expect("FFmpeg build process: make compile failed!");

        Command::new("make")
            .current_dir(ffmpeg_dir)
            .arg(format!("-j{}", num_cpus))
            .arg("install")
            .spawn()
            .expect("FFmpeg build process: make install failed!")
            .wait()
            .expect("FFmpeg build process: make install failed!");
    }

    fn link_libraries(
        library_names: &[&str],
        ffmpeg_pkg_config_path: Option<String>,
        is_static: bool,
    ) -> HashSet<path::PathBuf> {
        let previous_pkg_config_path = env::var("PKG_CONFIG_PATH").ok();
        if let Some(path) = ffmpeg_pkg_config_path {
            // for pkg-config
            env::set_var("PKG_CONFIG_PATH", path);
        } else {
            env::remove_var("PKG_CONFIG_PATH");
        }
        // TODO: if specific library is not enabled, we should not probe it. If we
        // want to implement this, we Should modify try_probe_system_ffmpeg() too.
        let include_paths = library_names
            .iter()
            .map(|libname| {
                pkg_config::Config::new()
                    // currently only support building with static libraries.
                    .statik(is_static)
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

        if let Some(path) = previous_pkg_config_path {
            env::set_var("PKG_CONFIG_PATH", path);
        } else {
            env::remove_var("PKG_CONFIG_PATH");
        }

        include_paths
    }

    pub fn link_ffmpeg(
        library_names: &[&str],
        out_dir: &str,
        is_static: bool,
    ) -> HashSet<path::PathBuf> {
        let ffmpeg_pkg_config_path = match env::var("FFMPEG_PKG_CONFIG_PATH") {
            Ok(x) => Some(x),
            Err(_) => {
                match try_probe_system_ffmpeg(library_names) {
                    Ok(_) => None,
                    Err(libname) => {
                        // If no system FFmpeg found, download and build one
                        eprintln!(
                            "{} not found in system path, let's git clone it and build.",
                            libname
                        );
                        clone_and_build_ffmpeg(out_dir);
                        Some(format!("{}/ffmpeg/build/lib/pkgconfig", out_dir))
                    }
                }
            }
        };
        // Now we can ensure available FFmpeg libraries.

        // Probe libraries(enable emitting cargo metadata)
        link_libraries(library_names, ffmpeg_pkg_config_path, is_static)
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    pub fn link_ffmpeg(
        _library_names: &[&str],
        _out_dir: &str,
        is_static: bool,
    ) -> HashSet<path::PathBuf> {
        let vcpkgrs_dynamic = env::var("VCPKGRS_DYNAMIC").ok();
        if is_static {
            env::remove_var("VCPKGRS_DYNAMIC");
        } else {
            env::set_var("VCPKGRS_DYNAMIC", "1");
        }

        let include_paths = vcpkg::Config::new()
            .find_package("ffmpeg")
            .unwrap()
            .include_paths
            .into_iter()
            .collect();

        if let Some(x) = vcpkgrs_dynamic {
            env::set_var("VCPKGRS_DYNAMIC", x);
        } else {
            env::remove_var("VCPKGRS_DYNAMIC");
        }
        include_paths
    }
}

fn main() {
    /* Workaround of cargo rerun-if-env-changed bug
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=VCPKG_ROOT");
    println!("cargo:rerun-if-env-changed=FFMPEG_PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=FFMPEG_DYNAMIC_LINKING");
    */

    let out_dir = &env::var("OUT_DIR").unwrap();

    let binding_file_path = &format!("{}/binding.rs", out_dir);

    // If it's a documentation generation from docs.rs, just copy the bindings
    // generated locally to `OUT_DIR`. We do this because the building
    // environment of docs.rs doesn't have an network connection, so we cannot
    // git clone the FFmpeg. And they also have a limitation on crate's size:
    // 10MB, which is not enough to fit in FFmpeg source files. So the only
    // thing we can do is copying the locally generated binding files to the
    // `OUT_DIR`.
    if env::var("DOCS_RS").is_ok() {
        fs::copy("src/binding.rs", binding_file_path)
            .expect("Prebuilt binding file failed to be copied.");
        return;
    }

    let is_static = env::var("FFMPEG_DYNAMIC_LINKING").is_err();

    #[cfg(not(target_os = "windows"))]
    use non_windows::link_ffmpeg;
    #[cfg(target_os = "windows")]
    use windows::link_ffmpeg;
    let include_paths = link_ffmpeg(&*LIBS, out_dir, is_static);

    generate_bindings(&include_paths, &*HEADERS)
        .expect("Binding generation failed.")
        // Is it correct to generate binding to one file? :-/
        .write_to_file(binding_file_path)
        .expect("Cannot write binding to file.")
}
