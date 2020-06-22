//! Port from Original code: https://github.com/leandromoreira/ffmpeg-libav-tutorial/blob/master/0_hello_world.c
//! Since this is a ported code, many warnings will emits.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use rusty_ffmpeg::ffi::*;

use libc::c_int;
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Write,
    mem::size_of,
    ptr::null_mut,
    slice,
};

#[inline(always)]
pub fn AVERROR(e: c_int) -> c_int {
    -e
}

#[macro_export]
macro_rules! MKTAG {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as isize) | (($b as isize) << 8) | (($c as isize) << 16) | (($d as isize) << 24)
    };
}

macro_rules! FFERRTAG {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        -MKTAG!($a, $b, $c, $d) as c_int
    };
}

pub const AVERROR_BSF_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'B', b'S', b'F');
pub const AVERROR_BUG: c_int = FFERRTAG!(b'B', b'U', b'G', b'!');
pub const AVERROR_BUFFER_TOO_SMALL: c_int = FFERRTAG!(b'B', b'U', b'F', b'S');
pub const AVERROR_DECODER_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'D', b'E', b'C');
pub const AVERROR_DEMUXER_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'D', b'E', b'M');
pub const AVERROR_ENCODER_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'E', b'N', b'C');
pub const AVERROR_EOF: c_int = FFERRTAG!(b'E', b'O', b'F', b' ');
pub const AVERROR_EXIT: c_int = FFERRTAG!(b'E', b'X', b'I', b'T');
pub const AVERROR_EXTERNAL: c_int = FFERRTAG!(b'E', b'X', b'T', b' ');
pub const AVERROR_FILTER_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'F', b'I', b'L');
pub const AVERROR_INVALIDDATA: c_int = FFERRTAG!(b'I', b'N', b'D', b'A');
pub const AVERROR_MUXER_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'M', b'U', b'X');
pub const AVERROR_OPTION_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'O', b'P', b'T');
pub const AVERROR_PATCHWELCOME: c_int = FFERRTAG!(b'P', b'A', b'W', b'E');
pub const AVERROR_PROTOCOL_NOT_FOUND: c_int = FFERRTAG!(0xF8, b'P', b'R', b'O');

fn main() {
    let filepath: CString = CString::new("./examples/slice/bear.mp4").unwrap();

    println!("initializing all the containers, codecs and protocols.");

    let pFormatContext = unsafe { avformat_alloc_context().as_mut() }
        .expect("ERROR could not allocate memory for Format Context");

    println!(
        "opening the input file ({}) and loading format (container) header",
        filepath.to_str().unwrap()
    );

    let result = unsafe {
        avformat_open_input(
            &mut (pFormatContext as *mut _),
            filepath.as_ptr(),
            null_mut(),
            null_mut(),
        )
    };

    if result != 0 {
        panic!("ERROR could not open the file");
    }

    let format_name = unsafe { CStr::from_ptr((*pFormatContext.iformat).name) };
    let format_name = format_name.to_str().unwrap();

    println!(
        "format {}, duration {} us, bit_rate {}",
        format_name, pFormatContext.duration, pFormatContext.bit_rate
    );

    println!("finding stream info from format");

    let result = unsafe { avformat_find_stream_info(pFormatContext, null_mut()) };
    if result < 0 {
        panic!("ERROR could not get the stream info");
    }

    let mut pCodec: *const AVCodec = null_mut();
    let mut pCodecParameters: *const AVCodecParameters = null_mut();
    let mut video_stream_index = None;

    for i in 0..pFormatContext.nb_streams as i32 {
        let stream = unsafe {
            slice::from_raw_parts(pFormatContext.streams, size_of::<AVStream>())[i as usize]
                .as_ref()
        }
        .unwrap();
        let pLocalCodecParameters = unsafe { stream.codecpar.as_ref() }.unwrap();
        println!(
            "AVStream->time_base before open coded {}/{}",
            stream.time_base.num, stream.time_base.den
        );
        println!(
            "AVStream->r_frame_rate before open coded {}/{}",
            stream.r_frame_rate.num, stream.r_frame_rate.den
        );
        println!("AVStream->start_time {}", stream.start_time);
        println!("AVStream->duration {}", stream.duration);
        println!("finding the proper decoder (CODEC)");

        let pLocalCodec = unsafe { avcodec_find_decoder(pLocalCodecParameters.codec_id).as_ref() }
            .expect("ERROR unsupported codec!");

        if pLocalCodecParameters.codec_type == AVMediaType_AVMEDIA_TYPE_VIDEO {
            if video_stream_index.is_none() {
                video_stream_index = Some(i);
                pCodec = pLocalCodec;
                pCodecParameters = pLocalCodecParameters;
            }

            println!(
                "Video Codec: resolution {} x {}",
                pLocalCodecParameters.width, pLocalCodecParameters.height
            );
        } else if pLocalCodecParameters.codec_type == AVMediaType_AVMEDIA_TYPE_AUDIO {
            println!(
                "Audio Codec: {} channels, sample rate {}",
                pLocalCodecParameters.channels, pLocalCodecParameters.sample_rate
            );
        }

        let codec_name = unsafe { CStr::from_ptr(pLocalCodec.name) };

        let codec_name = codec_name.to_str().unwrap();

        println!(
            "\tCodec {} ID {} bit_rate {}",
            codec_name, pLocalCodec.id, pLocalCodecParameters.bit_rate
        );
    }
    let pCodecContext = unsafe { avcodec_alloc_context3(pCodec).as_mut() }
        .expect("failed to allocated memory for AVCodecContext");

    let result = unsafe { avcodec_parameters_to_context(pCodecContext, pCodecParameters) };
    if result < 0 {
        panic!("failed to copy codec params to codec context");
    }

    let result = unsafe { avcodec_open2(pCodecContext, pCodec, null_mut()) };
    if result < 0 {
        panic!("failed to open codec through avcodec_open2");
    }

    let pFrame =
        unsafe { av_frame_alloc().as_mut() }.expect("failed to allocated memory for AVFrame");
    let pPacket =
        unsafe { av_packet_alloc().as_mut() }.expect("failed to allocated memory for AVPacket");

    let mut how_many_packets_to_process = 8;

    while unsafe { av_read_frame(pFormatContext, pPacket) } >= 0 {
        if Some(pPacket.stream_index) == video_stream_index {
            println!("AVPacket->pts {}", pPacket.pts);
            if let Err(s) = decode_packet(pPacket, pCodecContext, pFrame) {
                panic!(s);
            }
            how_many_packets_to_process -= 1;
            if how_many_packets_to_process <= 0 {
                break;
            }
        }
        unsafe { av_packet_unref(pPacket) };
    }

    println!("releasing all the resources");

    unsafe {
        avformat_close_input(&mut (pFormatContext as *mut _));
        av_packet_free(&mut (pPacket as *mut _));
        av_frame_free(&mut (pFrame as *mut _));
        avcodec_free_context(&mut (pCodecContext as *mut _));
    }
}

fn decode_packet(
    pPacket: &AVPacket,
    pCodecContext: &mut AVCodecContext,
    pFrame: &mut AVFrame,
) -> Result<(), String> {
    let response = unsafe { avcodec_send_packet(pCodecContext, pPacket) };

    if response < 0 {
        return Err(String::from("Error while sending a packet to the decoder."));
    }

    while response >= 0 {
        let response = unsafe { avcodec_receive_frame(pCodecContext, pFrame) };
        if response == AVERROR(EAGAIN as i32) || response == AVERROR_EOF {
            break;
        } else if response < 0 {
            return Err(String::from(
                "Error while receiving a frame from the decoder.",
            ));
        }

        if response >= 0 {
            println!(
                "Frame {} (type={}, size={} bytes) pts {} key_frame {} [DTS {}]",
                pCodecContext.frame_number,
                unsafe { av_get_picture_type_char(pFrame.pict_type) },
                pFrame.pkt_size,
                pFrame.pts,
                pFrame.key_frame,
                pFrame.coded_picture_number
            );

            let frame_filename = format!(
                "./examples/slice/output/frame-{}.pgm",
                pCodecContext.frame_number
            );
            let width = pFrame.width as usize;
            let height = pFrame.height as usize;
            let wrap = pFrame.linesize[0] as usize;
            let data = unsafe { slice::from_raw_parts(pFrame.data[0], wrap * height) };
            save_gray_frame(data, wrap, width, height, frame_filename);
        }
    }
    Ok(())
}

fn save_gray_frame(buf: &[u8], wrap: usize, xsize: usize, ysize: usize, filename: String) -> bool {
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(_) => return false,
    };
    let data = format!("P5\n{} {}\n{}\n", xsize, ysize, 255);
    if let Err(_) = file.write_all(data.as_bytes()) {
        return false;
    }

    for i in 0..ysize {
        if let Err(_) = file.write_all(&buf[i * wrap..(i * wrap + xsize)]) {
            return false;
        }
    }
    true
}
