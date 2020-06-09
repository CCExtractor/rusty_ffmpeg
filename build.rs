#![feature(bool_to_option)]
use bindgen::{self, CargoCallbacks};
use once_cell::sync::{Lazy, OnceCell};
use pkg_config as pkgconfig;

use std::{collections::HashSet, convert::From, env, fs, path};

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

fn out_dir() -> path::PathBuf {
    let x = OnceCell::new();
    x.get_or_init(|| path::PathBuf::from(env::var("OUT_DIR").unwrap()))
        .clone()
}

fn main() {
    // We currently only support building with static libraries.

    /* Thanks to pkg-config, we almost don't need this.
    // Output link libraries
    (&*LIBS)
        .iter()
        .for_each(|name| println!("cargo:rustc-link-lib={}={}", "static", name));
    */

    // Probe libraries
    // TODO if not enabled, we should not probe it
    let include_paths = (&*LIBS)
        .iter()
        .map(|name| "lib".to_owned() + name)
        .map(|libname| {
            pkgconfig::Config::new()
                .statik(true)
                .probe(&libname)
                .expect(&format!("{} not found!", libname))
                .include_paths
        })
        .fold(HashSet::new(), |mut acc, paths| {
            paths.into_iter().for_each(|path| {
                acc.insert(path);
            });
            acc
        });
    // TODO: need this be panic?
    let include_path = if include_paths.len() > 1 {
        panic!("Inconsistent include paths");
    } else {
        include_paths.iter().next().expect("No include_path.")
    };

    // TODO mysterious feature checking should be done

    // Bindgen the headers
    let builder = (&*LIBS)
        .iter()
        // TODO if not enabled, we should not manipulate it, consider adding a filter here
        .map(|name| "lib".to_owned() + name)
        .map(|libname| {
            let mut path = path::PathBuf::from(include_path);
            path.push(&libname);
            (libname, path.into_os_string())
        })
        .fold(
            // For each library(e.g. libavcodec), find all headers under its folder and `Builder::header()` it.
            bindgen::builder().parse_callbacks(Box::new(CargoCallbacks)),
            |builder, (libname, path)| {
                fs::read_dir(&path)
                    .expect(&format!("Cannot open libfolder:{}", libname))
                    .map(|entry| entry.unwrap())
                    // Filter out all entries which is file
                    .filter_map(|entry| entry.file_type().unwrap().is_file().then_some(entry))
                    // Filter out all files which name ends with `.h`
                    .filter_map(|entry| {
                        let name = entry.file_name();
                        name.to_string_lossy().ends_with(".h").then_some(name)
                    })
                    // Builder binds header files
                    .fold(builder, |builder, name| {
                        let file_path: path::PathBuf = [path.clone(), name].iter().collect();
                        builder.header(file_path.to_string_lossy())
                    })
            },
        );

    // Is it correct to generate binding to one file? :-/
    let output_path: path::PathBuf = [out_dir(), "binding.rs".into()].iter().collect();
    builder
        .generate()
        .expect("Binding generation failed.")
        .write_to_file(output_path)
        .expect("Cannot write binding to file!")
}
