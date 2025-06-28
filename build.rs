use bindgen::RustTarget;
use bindgen::{callbacks, Bindings};
use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use once_cell::sync::Lazy;
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
        "libavcodec/ac3_parser.h",
        "libavcodec/adts_parser.h",
        "libavcodec/avcodec.h",
        "libavcodec/avdct.h",
        "libavcodec/avfft.h",
        "libavcodec/bsf.h",
        "libavcodec/codec.h",
        "libavcodec/codec_desc.h",
        "libavcodec/codec_id.h",
        "libavcodec/codec_par.h",
        // "libavcodec/d3d11va.h",
        "libavcodec/defs.h",
        "libavcodec/dirac.h",
        "libavcodec/dv_profile.h",
        // "libavcodec/dxva2.h",
        "libavcodec/jni.h",
        "libavcodec/mediacodec.h",
        "libavcodec/packet.h",
        // "libavcodec/qsv.h",
        // "libavcodec/vdpau.h",
        "libavcodec/version.h",
        "libavcodec/version_major.h",
        // "libavcodec/videotoolbox.h",
        "libavcodec/vorbis_parser.h",
        // "libavcodec/xvmc.h",
        "libavdevice/avdevice.h",
        "libavdevice/version.h",
        "libavdevice/version_major.h",
        "libavfilter/avfilter.h",
        "libavfilter/buffersink.h",
        "libavfilter/buffersrc.h",
        "libavfilter/version.h",
        "libavfilter/version_major.h",
        "libavformat/avformat.h",
        "libavformat/avio.h",
        "libavformat/version.h",
        "libavformat/version_major.h",
        "libavutil/adler32.h",
        "libavutil/aes.h",
        "libavutil/aes_ctr.h",
        "libavutil/ambient_viewing_environment.h",
        "libavutil/attributes.h",
        "libavutil/audio_fifo.h",
        "libavutil/avassert.h",
        "libavutil/avconfig.h",
        "libavutil/avstring.h",
        "libavutil/avutil.h",
        "libavutil/base64.h",
        "libavutil/blowfish.h",
        "libavutil/bprint.h",
        "libavutil/bswap.h",
        "libavutil/buffer.h",
        "libavutil/camellia.h",
        "libavutil/cast5.h",
        "libavutil/channel_layout.h",
        "libavutil/common.h",
        "libavutil/cpu.h",
        "libavutil/crc.h",
        "libavutil/csp.h",
        "libavutil/des.h",
        "libavutil/detection_bbox.h",
        "libavutil/dict.h",
        "libavutil/display.h",
        "libavutil/dovi_meta.h",
        "libavutil/downmix_info.h",
        "libavutil/encryption_info.h",
        "libavutil/error.h",
        "libavutil/eval.h",
        "libavutil/executor.h",
        "libavutil/ffversion.h",
        "libavutil/fifo.h",
        "libavutil/file.h",
        "libavutil/film_grain_params.h",
        "libavutil/frame.h",
        "libavutil/hash.h",
        "libavutil/hdr_dynamic_metadata.h",
        "libavutil/hdr_dynamic_vivid_metadata.h",
        "libavutil/hmac.h",
        "libavutil/hwcontext.h",
        // "libavutil/hwcontext_cuda.h",
        // "libavutil/hwcontext_d3d11va.h",
        // "libavutil/hwcontext_drm.h",
        // "libavutil/hwcontext_dxva2.h",
        // "libavutil/hwcontext_mediacodec.h",
        // "libavutil/hwcontext_opencl.h",
        // "libavutil/hwcontext_qsv.h",
        // "libavutil/hwcontext_vaapi.h",
        // "libavutil/hwcontext_vdpau.h",
        // "libavutil/hwcontext_videotoolbox.h",
        // "libavutil/hwcontext_vulkan.h",
        "libavutil/imgutils.h",
        "libavutil/intfloat.h",
        "libavutil/intreadwrite.h",
        "libavutil/lfg.h",
        "libavutil/log.h",
        "libavutil/lzo.h",
        "libavutil/macros.h",
        "libavutil/mastering_display_metadata.h",
        "libavutil/mathematics.h",
        "libavutil/md5.h",
        "libavutil/mem.h",
        "libavutil/motion_vector.h",
        "libavutil/murmur3.h",
        "libavutil/opt.h",
        "libavutil/parseutils.h",
        "libavutil/pixdesc.h",
        "libavutil/pixelutils.h",
        "libavutil/pixfmt.h",
        "libavutil/random_seed.h",
        "libavutil/rational.h",
        "libavutil/rc4.h",
        "libavutil/replaygain.h",
        "libavutil/ripemd.h",
        "libavutil/samplefmt.h",
        "libavutil/sha.h",
        "libavutil/sha512.h",
        "libavutil/spherical.h",
        "libavutil/stereo3d.h",
        "libavutil/tea.h",
        "libavutil/threadmessage.h",
        "libavutil/time.h",
        "libavutil/timecode.h",
        "libavutil/timestamp.h",
        "libavutil/tree.h",
        "libavutil/twofish.h",
        "libavutil/tx.h",
        "libavutil/uuid.h",
        "libavutil/version.h",
        "libavutil/video_enc_params.h",
        "libavutil/video_hint.h",
        "libavutil/xtea.h",
        "libswresample/swresample.h",
        "libswresample/version.h",
        "libswresample/version_major.h",
        "libswscale/swscale.h",
        "libswscale/version.h",
        "libswscale/version_major.h",
    ]
    .into_iter()
    .map(|x| Path::new(x).into_iter().collect())
    .collect()
});

/// Filter out all symbols in the HashSet, and for others things it will act
/// exactly the same as `CargoCallback`.
#[derive(Debug)]
struct FilterCargoCallbacks {
    emitted_macro: HashSet<&'static str>,
}

impl FilterCargoCallbacks {
    fn new(set: HashSet<&'static str>) -> Self {
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

fn generate_bindings(ffmpeg_include_dir: &Path, headers: &[PathBuf]) -> Bindings {
    if !Path::new(ffmpeg_include_dir).exists() {
        panic!(
            "FFmpeg include dir: `{:?}` doesn't exits",
            ffmpeg_include_dir
        );
    }
    // Because of the strange `FP_*` in `math.h` https://github.com/rust-lang/rust-bindgen/issues/687
    let filter_callback = FilterCargoCallbacks::new(
        [
            "FP_NAN",
            "FP_INFINITE",
            "FP_ZERO",
            "FP_SUBNORMAL",
            "FP_NORMAL",
        ]
        .into_iter()
        .collect(),
    );

    // Bindgen on all avaiable headers
    headers
        .iter()
        .map(|header| ffmpeg_include_dir.join(header))
        .filter(|path| {
            let exists = Path::new(&path).exists();
            if !exists {
                eprintln!("Header path `{:?}` not found.", path);
            }
            exists
        })
        .fold(
            {
                bindgen::builder()
                    // Force impl Debug if possible(for `AVCodecParameters`)
                    .impl_debug(true)
                    .rust_target(RustTarget::stable(68, 0).ok().unwrap())
                    .parse_callbacks(Box::new(filter_callback))
                    // Add clang path, for `#include` header finding in bindgen process.
                    .clang_arg(format!("-I{}", ffmpeg_include_dir))
                    // Workaround: https://github.com/rust-lang/rust-bindgen/issues/2159
                    .blocklist_type("__mingw_ldbl_type_t")
                    // Stop bindgen from prefixing enums
                    .prepend_enum_name(false)
            },
            |builder, header| builder.header(header),
        )
        .generate()
        .expect("Binding generation failed.")
}

fn linking_with_libs_dir(library_names: &[&str], ffmpeg_libs_dir: &Path, mode: FfmpegLinkMode) {
    println!("cargo:rustc-link-search=native={}", ffmpeg_libs_dir);
    for library_name in library_names {
        println!("cargo:rustc-link-lib={}={}", library_name, match mode {
            FfmpegLinkMode::Dynamic => "dylib",
            FfmpegLinkMode::Static => "static",
        });
    }
}

#[derive(Clone, Copy)]
enum FfmpegLinkMode {
    Static,
    Dynamic,
}

#[allow(dead_code)]
pub struct EnvVars {
    docs_rs: Option<String>,
    out_dir: Option<PathBuf>,
    ffmpeg_include_dir: Option<PathBuf>,
    ffmpeg_link_mode: Option<FfmpegLinkMode>,
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
            ffmpeg_link_mode: match env::var("FFMPEG_LINK_MODE").ok().as_deref() {
                Some("static") => Some(FfmpegLinkMode::Static),
                Some("dynamic") => Some(FfmpegLinkMode::Dynamic),
                Some(r) => panic!("invalid FFMPEG_LINK_MODE value {r}, expected [static,dynamic]"),
                None => None,
            }
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
mod pkg_config_linking {
    use super::*;

    /// Returns error when some library are missing. Otherwise, returns the paths of the libraries.
    ///
    /// Note: no side effect if this function errors.
    pub fn linking_with_pkg_config(
        library_names: &[&str],
    ) -> Result<Vec<PathBuf>, pkg_config::Error> {
        // dry run for library linking
        for libname in library_names {
            pkg_config::Config::new()
                .cargo_metadata(false)
                .env_metadata(false)
                .print_system_libs(false)
                .print_system_cflags(false)
                .probe(&format!("lib{}", libname))?;
        }

        // real linking
        let mut paths = HashSet::new();
        for libname in library_names {
            let new_paths = pkg_config::Config::new()
                .probe(&format!("lib{}", libname))
                .unwrap_or_else(|_| panic!("{} not found!", libname))
                .include_paths;
            for new_path in new_paths {
                let new_path = new_path.to_str().unwrap().to_string();
                paths.insert(new_path);
            }
        }
        Ok(paths.into_iter().map(PathBuf::from).collect())
    }
}

#[cfg(feature = "link_vcpkg_ffmpeg")]
mod vcpkg_linking {
    use super::*;

    fn linking_with_vcpkg(
        _env_vars: &EnvVars,
        _library_names: &[&str],
    ) -> Result<Vec<PathBuf>, vcpkg::Error> {
        Ok(vcpkg::Config::new()
            .find_package("ffmpeg")?
            .include_paths
            .into_iter()
            .map(|x| PathBuf::from_path_buf(x).unwrap())
            .collect())
    }

    pub fn linking_with_vcpkg_and_bindgen(
        env_vars: &EnvVars,
        output_binding_path: &Path,
    ) -> Result<(), vcpkg::Error> {
        let include_paths = linking_with_vcpkg(env_vars, &*LIBS)?;
        if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
            use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
        } else {
            generate_bindings(&include_paths[0], &HEADERS)
                .write_to_file(output_binding_path)
                .expect("Cannot write binding to file.");
        }
        Ok(())
    }
}

fn dynamic_linking(mut env_vars: EnvVars) {
    let ffmpeg_dll_path = env_vars.ffmpeg_dll_path.as_ref().unwrap();
    if ffmpeg_dll_path.is_dir() {
        if env_vars.ffmpeg_libs_dir.is_none() {
            env_vars.ffmpeg_libs_dir = Some(ffmpeg_dll_path.clone());
        }
        if env_vars.ffmpeg_link_mode.is_none() {
            env_vars.ffmpeg_link_mode = Some(FfmpegLinkMode::Dynamic);
        }

        return linking(env_vars);
    }

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
        generate_bindings(ffmpeg_include_dir, &HEADERS)
            // Is it correct to generate binding to one file? :-/
            .write_to_file(output_binding_path)
            .expect("Cannot write binding to file.");
    } else {
        panic!("No binding generation method is set!");
    }
}

fn linking(env_vars: EnvVars) {
    let output_binding_path = &env_vars.out_dir.as_ref().unwrap().join("binding.rs");

    #[cfg(not(target_os = "windows"))]
    {
        fn linking_with_pkg_config_and_bindgen(
            env_vars: &EnvVars,
            output_binding_path: &Path,
        ) -> Result<(), pkg_config::Error> {
            // Probe libraries(enable emitting cargo metadata)
            let include_paths = pkg_config_linking::linking_with_pkg_config(&*LIBS)?;
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                // If use ffmpeg_pkg_config_path with ffmpeg_include_dir, prefer using the user given dir rather than pkg_config_path.
                generate_bindings(ffmpeg_include_dir, &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                generate_bindings(&include_paths[0], &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            }
            Ok(())
        }
        // Hint: set PKG_CONFIG_PATH to some placeholder value will let pkg_config probing system library.
        if let Some(ffmpeg_pkg_config_path) = env_vars.ffmpeg_pkg_config_path.as_ref() {
            if !Path::new(ffmpeg_pkg_config_path).exists() {
                panic!(
                    "error: FFMPEG_PKG_CONFIG_PATH is set to `{}`, which does not exist.",
                    ffmpeg_pkg_config_path
                );
            }
            env::set_var("PKG_CONFIG_PATH", ffmpeg_pkg_config_path);
            linking_with_pkg_config_and_bindgen(&env_vars, output_binding_path)
                .expect("Static linking with pkg-config failed.");
        } else if let Some(ffmpeg_libs_dir) = env_vars.ffmpeg_libs_dir.as_ref() {
            linking_with_libs_dir(&*LIBS, ffmpeg_libs_dir, env_vars.ffmpeg_link_mode.unwrap_or(FfmpegLinkMode::Static));
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                generate_bindings(ffmpeg_include_dir, &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                panic!("No binding generation method is set!");
            }
        } else {
            #[cfg(not(any(feature = "link_system_ffmpeg", feature = "link_vcpkg_ffmpeg")))]
            panic!(
                "
!!!!!!! rusty_ffmpeg: No linking method set!
Use `FFMPEG_PKG_CONFIG_PATH` or `FFMPEG_LIBS_DIR` if you have prebuilt FFmpeg libraries.
Enable `link_system_ffmpeg` feature if you want to link ffmpeg libraries installed in system path(which can be probed by pkg-config).
Enable `link_vcpkg_ffmpeg` feature if you want to link ffmpeg libraries installed by vcpkg.
"
            );
            #[cfg(any(feature = "link_system_ffmpeg", feature = "link_vcpkg_ffmpeg"))]
            {
                let mut success = false;
                let mut error = String::new();
                #[cfg(feature = "link_system_ffmpeg")]
                if !success {
                    if let Err(e) =
                        linking_with_pkg_config_and_bindgen(env_vars, output_binding_path)
                    {
                        error.push('\n');
                        error.push_str(&format!("Link system FFmpeg failed: {:?}", e));
                    } else {
                        println!("Link system FFmpeg succeeded.");
                        success = true;
                    }
                }
                #[cfg(feature = "link_vcpkg_ffmpeg")]
                if !success {
                    if let Err(e) =
                        vcpkg_linking::linking_with_vcpkg_and_bindgen(env_vars, output_binding_path)
                    {
                        error.push('\n');
                        error.push_str(&format!("Link vcpkg FFmpeg failed: {:?}", e));
                    } else {
                        println!("Link vcpkg FFmpeg succeeded.");
                        success = true;
                    }
                }
                if !success {
                    panic!("FFmpeg linking trial failed: {}", error);
                }
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(ffmpeg_libs_dir) = env_vars.ffmpeg_libs_dir.as_ref() {
            linking_with_libs_dir(&*LIBS, ffmpeg_libs_dir, env_vars.ffmpeg_link_mode);
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                generate_bindings(ffmpeg_include_dir, &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                panic!("No binding generation method is set!");
            }
        } else {
            #[cfg(feature = "link_vcpkg_ffmpeg")]
            vcpkg_linking::linking_with_vcpkg_and_bindgen(&zenv_vars, output_binding_path)
                .expect("Linking FFmpeg with vcpkg failed.");
            #[cfg(not(feature = "link_vcpkg_ffmpeg"))]
            panic!(
                "
!!!!!!! rusty_ffmpeg: No linking method set!
Use FFMPEG_LIBS_DIR if you have prebuilt FFmpeg libraries.
Enable `link_vcpkg_ffmpeg` feature if you want to link ffmpeg provided by vcpkg.
"
            );
        }
    }
}

fn docs_rs_linking(env_vars: EnvVars) {
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
        docs_rs_linking(env_vars);
    } else if env_vars.ffmpeg_dll_path.is_some() {
        dynamic_linking(env_vars);
    } else {
        // fallback to static linking
        linking(env_vars);
    }
}
