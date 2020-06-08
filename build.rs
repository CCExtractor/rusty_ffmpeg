use bindgen;
use once_cell::sync::Lazy;

static LIBS: Lazy<[&str; 9]> = Lazy::new(|| {
    [
        "avcodec",
        "avdevice",
        "avfilter",
        "avformat",
        "avresample",
        "avutil",
        "postproc",
        "swresample",
        "swscale",
    ]
});

fn main() {
    // We currently only support building with static libraries.
    (&*LIBS).iter().for_each(|name| {});
}
