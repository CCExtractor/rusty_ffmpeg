#![allow(non_upper_case_globals)]
#![rustfmt::skip]

use crate::ffi::*;

macro_rules! AV_PIX_FMT_NE {
    ($def: ident, $be: ident, $le: ident) => {
        #[cfg(target_endian = "big")]
        pub const $def: AVPixelFormat = $be;

        #[cfg(target_endian = "little")]
        pub const $def: AVPixelFormat = $le;
    };
}

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGB32, AVPixelFormat_AV_PIX_FMT_ARGB, AVPixelFormat_AV_PIX_FMT_BGRA);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGB32_1, AVPixelFormat_AV_PIX_FMT_RGBA, AVPixelFormat_AV_PIX_FMT_ABGR);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGR32, AVPixelFormat_AV_PIX_FMT_ABGR, AVPixelFormat_AV_PIX_FMT_RGBA);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGR32_1, AVPixelFormat_AV_PIX_FMT_BGRA, AVPixelFormat_AV_PIX_FMT_ARGB);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_0RGB32, AVPixelFormat_AV_PIX_FMT_0RGB, AVPixelFormat_AV_PIX_FMT_BGR0);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_0BGR32, AVPixelFormat_AV_PIX_FMT_0BGR, AVPixelFormat_AV_PIX_FMT_RGB0);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GRAY9, AVPixelFormat_AV_PIX_FMT_GRAY9BE, AVPixelFormat_AV_PIX_FMT_GRAY9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GRAY10, AVPixelFormat_AV_PIX_FMT_GRAY10BE, AVPixelFormat_AV_PIX_FMT_GRAY10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GRAY12, AVPixelFormat_AV_PIX_FMT_GRAY12BE, AVPixelFormat_AV_PIX_FMT_GRAY12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GRAY16, AVPixelFormat_AV_PIX_FMT_GRAY16BE, AVPixelFormat_AV_PIX_FMT_GRAY16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YA16, AVPixelFormat_AV_PIX_FMT_YA16BE, AVPixelFormat_AV_PIX_FMT_YA16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGB48, AVPixelFormat_AV_PIX_FMT_RGB48BE, AVPixelFormat_AV_PIX_FMT_RGB48LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGB565, AVPixelFormat_AV_PIX_FMT_RGB565BE, AVPixelFormat_AV_PIX_FMT_RGB565LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGB555, AVPixelFormat_AV_PIX_FMT_RGB555BE, AVPixelFormat_AV_PIX_FMT_RGB555LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGB444, AVPixelFormat_AV_PIX_FMT_RGB444BE, AVPixelFormat_AV_PIX_FMT_RGB444LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_RGBA64, AVPixelFormat_AV_PIX_FMT_RGBA64BE, AVPixelFormat_AV_PIX_FMT_RGBA64LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGR48, AVPixelFormat_AV_PIX_FMT_BGR48BE, AVPixelFormat_AV_PIX_FMT_BGR48LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGR565, AVPixelFormat_AV_PIX_FMT_BGR565BE, AVPixelFormat_AV_PIX_FMT_BGR565LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGR555, AVPixelFormat_AV_PIX_FMT_BGR555BE, AVPixelFormat_AV_PIX_FMT_BGR555LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGR444, AVPixelFormat_AV_PIX_FMT_BGR444BE, AVPixelFormat_AV_PIX_FMT_BGR444LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BGRA64, AVPixelFormat_AV_PIX_FMT_BGRA64BE, AVPixelFormat_AV_PIX_FMT_BGRA64LE);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV420P9, AVPixelFormat_AV_PIX_FMT_YUV420P9BE , AVPixelFormat_AV_PIX_FMT_YUV420P9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV422P9, AVPixelFormat_AV_PIX_FMT_YUV422P9BE , AVPixelFormat_AV_PIX_FMT_YUV422P9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV444P9, AVPixelFormat_AV_PIX_FMT_YUV444P9BE , AVPixelFormat_AV_PIX_FMT_YUV444P9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV420P10, AVPixelFormat_AV_PIX_FMT_YUV420P10BE, AVPixelFormat_AV_PIX_FMT_YUV420P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV422P10, AVPixelFormat_AV_PIX_FMT_YUV422P10BE, AVPixelFormat_AV_PIX_FMT_YUV422P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV440P10, AVPixelFormat_AV_PIX_FMT_YUV440P10BE, AVPixelFormat_AV_PIX_FMT_YUV440P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV444P10, AVPixelFormat_AV_PIX_FMT_YUV444P10BE, AVPixelFormat_AV_PIX_FMT_YUV444P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV420P12, AVPixelFormat_AV_PIX_FMT_YUV420P12BE, AVPixelFormat_AV_PIX_FMT_YUV420P12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV422P12, AVPixelFormat_AV_PIX_FMT_YUV422P12BE, AVPixelFormat_AV_PIX_FMT_YUV422P12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV440P12, AVPixelFormat_AV_PIX_FMT_YUV440P12BE, AVPixelFormat_AV_PIX_FMT_YUV440P12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV444P12, AVPixelFormat_AV_PIX_FMT_YUV444P12BE, AVPixelFormat_AV_PIX_FMT_YUV444P12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV420P14, AVPixelFormat_AV_PIX_FMT_YUV420P14BE, AVPixelFormat_AV_PIX_FMT_YUV420P14LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV422P14, AVPixelFormat_AV_PIX_FMT_YUV422P14BE, AVPixelFormat_AV_PIX_FMT_YUV422P14LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV444P14, AVPixelFormat_AV_PIX_FMT_YUV444P14BE, AVPixelFormat_AV_PIX_FMT_YUV444P14LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV420P16, AVPixelFormat_AV_PIX_FMT_YUV420P16BE, AVPixelFormat_AV_PIX_FMT_YUV420P16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV422P16, AVPixelFormat_AV_PIX_FMT_YUV422P16BE, AVPixelFormat_AV_PIX_FMT_YUV422P16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUV444P16, AVPixelFormat_AV_PIX_FMT_YUV444P16BE, AVPixelFormat_AV_PIX_FMT_YUV444P16LE);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRP9, AVPixelFormat_AV_PIX_FMT_GBRP9BE , AVPixelFormat_AV_PIX_FMT_GBRP9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRP10, AVPixelFormat_AV_PIX_FMT_GBRP10BE, AVPixelFormat_AV_PIX_FMT_GBRP10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRP12, AVPixelFormat_AV_PIX_FMT_GBRP12BE, AVPixelFormat_AV_PIX_FMT_GBRP12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRP14, AVPixelFormat_AV_PIX_FMT_GBRP14BE, AVPixelFormat_AV_PIX_FMT_GBRP14LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRP16, AVPixelFormat_AV_PIX_FMT_GBRP16BE, AVPixelFormat_AV_PIX_FMT_GBRP16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRAP10, AVPixelFormat_AV_PIX_FMT_GBRAP10BE, AVPixelFormat_AV_PIX_FMT_GBRAP10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRAP12, AVPixelFormat_AV_PIX_FMT_GBRAP12BE, AVPixelFormat_AV_PIX_FMT_GBRAP12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRAP16, AVPixelFormat_AV_PIX_FMT_GBRAP16BE, AVPixelFormat_AV_PIX_FMT_GBRAP16LE);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16, AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16BE, AVPixelFormat_AV_PIX_FMT_BAYER_BGGR16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16, AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16BE, AVPixelFormat_AV_PIX_FMT_BAYER_RGGB16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16, AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16BE, AVPixelFormat_AV_PIX_FMT_BAYER_GBRG16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16, AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16BE, AVPixelFormat_AV_PIX_FMT_BAYER_GRBG16LE);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRPF32, AVPixelFormat_AV_PIX_FMT_GBRPF32BE, AVPixelFormat_AV_PIX_FMT_GBRPF32LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_GBRAPF32, AVPixelFormat_AV_PIX_FMT_GBRAPF32BE, AVPixelFormat_AV_PIX_FMT_GBRAPF32LE);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA420P9, AVPixelFormat_AV_PIX_FMT_YUVA420P9BE , AVPixelFormat_AV_PIX_FMT_YUVA420P9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA422P9, AVPixelFormat_AV_PIX_FMT_YUVA422P9BE , AVPixelFormat_AV_PIX_FMT_YUVA422P9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA444P9, AVPixelFormat_AV_PIX_FMT_YUVA444P9BE , AVPixelFormat_AV_PIX_FMT_YUVA444P9LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA420P10, AVPixelFormat_AV_PIX_FMT_YUVA420P10BE, AVPixelFormat_AV_PIX_FMT_YUVA420P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA422P10, AVPixelFormat_AV_PIX_FMT_YUVA422P10BE, AVPixelFormat_AV_PIX_FMT_YUVA422P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA444P10, AVPixelFormat_AV_PIX_FMT_YUVA444P10BE, AVPixelFormat_AV_PIX_FMT_YUVA444P10LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA420P16, AVPixelFormat_AV_PIX_FMT_YUVA420P16BE, AVPixelFormat_AV_PIX_FMT_YUVA420P16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA422P16, AVPixelFormat_AV_PIX_FMT_YUVA422P16BE, AVPixelFormat_AV_PIX_FMT_YUVA422P16LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_YUVA444P16, AVPixelFormat_AV_PIX_FMT_YUVA444P16BE, AVPixelFormat_AV_PIX_FMT_YUVA444P16LE);

AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_XYZ12, AVPixelFormat_AV_PIX_FMT_XYZ12BE, AVPixelFormat_AV_PIX_FMT_XYZ12LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_NV20, AVPixelFormat_AV_PIX_FMT_NV20BE, AVPixelFormat_AV_PIX_FMT_NV20LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_AYUV64, AVPixelFormat_AV_PIX_FMT_AYUV64BE, AVPixelFormat_AV_PIX_FMT_AYUV64LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_P010, AVPixelFormat_AV_PIX_FMT_P010BE, AVPixelFormat_AV_PIX_FMT_P010LE);
AV_PIX_FMT_NE!(AVPixelFormat_AV_PIX_FMT_P016, AVPixelFormat_AV_PIX_FMT_P016BE, AVPixelFormat_AV_PIX_FMT_P016LE);
