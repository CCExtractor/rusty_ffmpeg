use bindgen::{self, callbacks, Bindings};
use once_cell::sync::Lazy;

use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use std::{collections::HashSet, env, fs};

/// All the libs that FFmpeg has
static LIBS: Lazy<[&str; 7]> = Lazy::new(|| {
    [
        "avcodec",
        "avdevice",
        "avfilter",
        "avformat",
        "avutil",
        "swresample",
        "swscale",
    ]
});

/// Whitelist of the headers we want to generate bindings
static HEADERS: Lazy<Vec<PathBuf>> = Lazy::new(|| {
    [
        "libavcodec/avcodec.h",
        "libavcodec/avfft.h",
        "libavcodec/bsf.h",
        "libavcodec/dv_profile.h",
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
        "libswresample/swresample.h",
        "libswscale/swscale.h",
    ]
    .into_iter()
    .map(|x| Path::new(x).into_iter().collect())
    .collect()
});

/// Filter out all symbols in the HashSet, and for others things it will act
/// exactly the same as `CargoCallback`.
#[derive(Debug)]
struct FilterCargoCallbacks {
    emitted_macro: HashSet<String>,
}

impl FilterCargoCallbacks {
    fn new(set: HashSet<String>) -> Self {
        Self { emitted_macro: set }
    }
}

impl callbacks::ParseCallbacks for FilterCargoCallbacks {
    fn will_parse_macro(&self, name: &str) -> callbacks::MacroParsingBehavior {
        if self.emitted_macro.contains(name) {
            callbacks::MacroParsingBehavior::Ignore
        } else {
            callbacks::MacroParsingBehavior::Default
        }
    }
}

fn use_prebuilt_binding(from: &Path, to: &Path) {
    fs::copy(from, to).expect("Prebuilt binding file failed to be copied.");
}

fn generate_bindings(ffmpeg_include_dir: Option<&Path>, headers: &[PathBuf]) -> Bindings {
    // Because of the strange `FP_*` in `math.h` https://github.com/rust-lang/rust-bindgen/issues/687
    let filter_callback = FilterCargoCallbacks::new(
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
            if let Some(ffmpeg_include_dir) = ffmpeg_include_dir {
                ffmpeg_include_dir.join(header)
            } else {
                header.clone()
            }
        })
        .fold(
            {
                let builder = bindgen::builder()
                    // Force impl Debug if possible(for `AVCodecParameters`)
                    .impl_debug(true)
                    .parse_callbacks(Box::new(filter_callback));
                if let Some(ffmpeg_include_dir) = ffmpeg_include_dir {
                    // Add clang path, for `#include` header finding in bindgen process.
                    builder.clang_arg(format!("-I{}", ffmpeg_include_dir))
                } else {
                    builder
                }
            },
            |builder, header| builder.header(header),
        )
        .generate()
        .expect("Binding generation failed.")
}

fn static_linking_with_libs_dir(library_names: &[&str], ffmpeg_libs_dir: &Path) {
    println!("cargo:rustc-link-search=native={}", ffmpeg_libs_dir);
    for library_name in library_names {
        println!("cargo:rustc-link-lib=static={}", library_name);
    }
}

#[allow(dead_code)]
pub struct EnvVars {
    docs_rs: Option<String>,
    out_dir: Option<PathBuf>,
    ffmpeg_include_dir: Option<PathBuf>,
    ffmpeg_dll_path: Option<PathBuf>,
    ffmpeg_pkg_config_path: Option<PathBuf>,
    ffmpeg_libs_dir: Option<PathBuf>,
    ffmpeg_binding_path: Option<PathBuf>,
}

impl EnvVars {
    fn init() -> Self {
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        println!("cargo:rerun-if-env-changed=OUT_DIR");
        println!("cargo:rerun-if-env-changed=FFMPEG_INCLUDE_DIR");
        println!("cargo:rerun-if-env-changed=FFMPEG_DLL_PATH");
        println!("cargo:rerun-if-env-changed=FFMPEG_PKG_CONFIG_PATH");
        println!("cargo:rerun-if-env-changed=FFMPEG_LIBS_DIR");
        println!("cargo:rerun-if-env-changed=FFMPEG_BINDING_PATH");
        Self {
            docs_rs: env::var("DOCS_RS").ok(),
            out_dir: env::var("OUT_DIR").ok().map(remove_verbatim),
            ffmpeg_include_dir: env::var("FFMPEG_INCLUDE_DIR").ok().map(remove_verbatim),
            ffmpeg_dll_path: env::var("FFMPEG_DLL_PATH").ok().map(remove_verbatim),
            ffmpeg_pkg_config_path: env::var("FFMPEG_PKG_CONFIG_PATH").ok().map(remove_verbatim),
            ffmpeg_libs_dir: env::var("FFMPEG_LIBS_DIR").ok().map(remove_verbatim),
            ffmpeg_binding_path: env::var("FFMPEG_BINDING_PATH").ok().map(remove_verbatim),
        }
    }
}

/// clang doesn't support -I{verbatim path} on windows, so we need to remove it if possible.
fn remove_verbatim(path: String) -> PathBuf {
    let path = if let Some(path) = path.strip_prefix(r#"\\?\"#) {
        path.to_string()
    } else {
        path
    };
    PathBuf::from(path)
}

#[cfg(not(target_os = "windows"))]
mod non_windows {
    use super::*;

    /// Try probing ffmpeg installed in system with no side effect. Return unfound Err(library name) when failed.
    // TODO: this is useful in static lib searching
    #[allow(dead_code)]
    fn try_probe_system_ffmpeg(library_names: &[&str]) -> Result<(), String> {
        match library_names.iter().find(|libname| {
            pkg_config::Config::new()
                // Remove side effect by disable metadata emitting
                .cargo_metadata(false)
                .probe(libname)
                .is_err()
        }) {
            Some(&libname) => Err(libname.to_string()),
            None => Ok(()),
        }
    }

    pub fn static_linking_with_pkg_config(
        library_names: &[&str],
        ffmpeg_pkg_config_path: &Path,
    ) -> Vec<PathBuf> {
        env::set_var("PKG_CONFIG_PATH", ffmpeg_pkg_config_path);
        // TODO: if specific library is not enabled, we should not probe it. If we
        // want to implement this, we Should modify try_probe_system_ffmpeg() too.
        let mut paths = HashSet::new();
        for libname in library_names {
            let new_paths = pkg_config::Config::new()
                // currently only support building with static libraries.
                .statik(true)
                .cargo_metadata(true)
                .probe(&format!("lib{}", libname))
                .unwrap_or_else(|_| panic!("{} not found!", libname))
                .include_paths;
            for new_path in new_paths {
                let new_path = new_path.to_str().unwrap().to_string();
                paths.insert(new_path);
            }
        }
        paths.into_iter().map(PathBuf::from).collect()
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    pub fn static_linking_vcpkg(_env_vars: &EnvVars, _library_names: &[&str]) -> Vec<PathBuf> {
        vcpkg::Config::new()
            .find_package("ffmpeg")
            .unwrap()
            .include_paths
            .into_iter()
            .map(|x| PathBuf::from_path_buf(x).unwrap())
            .collect()
    }
}

fn dynamic_linking(env_vars: &EnvVars) {
    let ffmpeg_dll_path = env_vars.ffmpeg_dll_path.as_ref().unwrap();

    let output_binding_path = &env_vars.out_dir.as_ref().unwrap().join("binding.rs");

    // Extract dll name and the dir the dll is in.
    let (ffmpeg_dll_name, ffmpeg_dll_dir) = {
        let mut ffmpeg_dll_path = PathBuf::from(ffmpeg_dll_path);
        // Without extension.
        let ffmpeg_dll_filename = ffmpeg_dll_path.file_stem().unwrap();
        let ffmpeg_dll_name = if cfg!(target_os = "windows") {
            ffmpeg_dll_filename
        } else {
            ffmpeg_dll_filename.trim_start_matches("lib")
        }
        .to_string();
        // Remove file name.
        ffmpeg_dll_path.pop();
        let ffmpeg_dll_path = ffmpeg_dll_path.to_string();
        (ffmpeg_dll_name, ffmpeg_dll_path)
    };

    println!("cargo:rustc-link-lib=dylib={}", ffmpeg_dll_name);
    println!("cargo:rustc-link-search=native={}", ffmpeg_dll_dir);

    if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
        use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
    } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
        generate_bindings(Some(ffmpeg_include_dir), &HEADERS)
            // Is it correct to generate binding to one file? :-/
            .write_to_file(output_binding_path)
            .expect("Cannot write binding to file.");
    } else {
        panic!("No binding generation method is set!");
    }
}

fn static_linking(env_vars: &EnvVars) {
    let output_binding_path = &env_vars.out_dir.as_ref().unwrap().join("binding.rs");

    #[cfg(not(target_os = "windows"))]
    {
        use non_windows::*;
        // Hint: set PKG_CONFIG_PATH to some placeholder value will let pkg_config probing system library.
        if let Some(ffmpeg_pkg_config_path) = env_vars.ffmpeg_pkg_config_path.as_ref() {
            // Probe libraries(enable emitting cargo metadata)
            let include_paths = static_linking_with_pkg_config(&*LIBS, ffmpeg_pkg_config_path);
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                // If use ffmpeg_pkg_config_path with ffmpeg_include_dir, prefer using the user given dir rather than pkg_config_path.
                generate_bindings(Some(ffmpeg_include_dir), &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                generate_bindings(Some(&include_paths[0]), &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            }
        } else if let Some(ffmpeg_libs_dir) = env_vars.ffmpeg_libs_dir.as_ref() {
            static_linking_with_libs_dir(&*LIBS, ffmpeg_libs_dir);
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                generate_bindings(Some(ffmpeg_include_dir), &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                panic!("No binding generation method is set!");
            }
        } else {
            panic!("No linking method set!");
        };
    }
    #[cfg(target_os = "windows")]
    {
        use windows::*;
        if let Some(ffmpeg_libs_dir) = env_vars.ffmpeg_libs_dir.as_ref() {
            static_linking_with_libs_dir(&*LIBS, ffmpeg_libs_dir);
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                generate_bindings(Some(ffmpeg_include_dir), &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                panic!("No binding generation method is set!");
            }
        } else {
            let include_paths = static_linking_vcpkg(env_vars, &*LIBS);
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else {
                generate_bindings(Some(&include_paths[0]), &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            }
        }
    }
}

fn docs_rs_linking(env_vars: &EnvVars) {
    // If it's a documentation generation from docs.rs, just copy the bindings
    // generated locally to `OUT_DIR`. We do this because the building
    // environment of docs.rs doesn't have an network connection, so we cannot
    // git clone the FFmpeg. And they also have a limitation on crate's size:
    // 10MB, which is not enough to fit in FFmpeg source files. So the only
    // thing we can do is copying the locally generated binding files to the
    // `OUT_DIR`.
    let binding_file_path = &env_vars.out_dir.as_ref().unwrap().join("binding.rs");
    use_prebuilt_binding(Path::new("src/binding.rs"), binding_file_path);
}

fn main() {
    let env_vars = EnvVars::init();
    if env_vars.docs_rs.is_some() {
        docs_rs_linking(&env_vars);
    } else if env_vars.ffmpeg_dll_path.is_some() {
        dynamic_linking(&env_vars);
    } else {
        // fallback to static linking
        static_linking(&env_vars);
    }
}
