//! Port from Original code: https://github.com/leandromoreira/ffmpeg-libav-tutorial/blob/master/0_hello_world.c
//! Since this is a ported code, many warnings will emits.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
use rusty_ffmpeg::{
    avutil::error::{AVERROR, AVERROR_EOF},
    ffi,
};

use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Write,
    mem::size_of,
    ptr::null_mut,
    slice,
};

fn main() {
    let filepath: CString = CString::new("./examples/slice/bear.mp4").unwrap();

    println!("initializing all the containers, codecs and protocols.");

    let pFormatContext = unsafe { ffi::avformat_alloc_context().as_mut() }
        .expect("ERROR could not allocate memory for Format Context");

    println!(
        "opening the input file ({}) and loading format (container) header",
        filepath.to_str().unwrap()
    );

    let result = unsafe {
        ffi::avformat_open_input(
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

    let result = unsafe { ffi::avformat_find_stream_info(pFormatContext, null_mut()) };
    if result < 0 {
        panic!("ERROR could not get the stream info");
    }

    let mut pCodec: *const ffi::AVCodec = null_mut();
    let mut pCodecParameters: *const ffi::AVCodecParameters = null_mut();
    let mut video_stream_index = None;

    for i in 0..pFormatContext.nb_streams as i32 {
        let stream = unsafe {
            slice::from_raw_parts(pFormatContext.streams, size_of::<ffi::AVStream>())[i as usize]
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

        let pLocalCodec =
            unsafe { ffi::avcodec_find_decoder(pLocalCodecParameters.codec_id).as_ref() }
                .expect("ERROR unsupported codec!");

        if pLocalCodecParameters.codec_type == ffi::AVMediaType_AVMEDIA_TYPE_VIDEO {
            if video_stream_index.is_none() {
                video_stream_index = Some(i);
                pCodec = pLocalCodec;
                pCodecParameters = pLocalCodecParameters;
            }

            println!(
                "Video Codec: resolution {} x {}",
                pLocalCodecParameters.width, pLocalCodecParameters.height
            );
        } else if pLocalCodecParameters.codec_type == ffi::AVMediaType_AVMEDIA_TYPE_AUDIO {
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
    let pCodecContext = unsafe { ffi::avcodec_alloc_context3(pCodec).as_mut() }
        .expect("failed to allocated memory for AVCodecContext");

    let result = unsafe { ffi::avcodec_parameters_to_context(pCodecContext, pCodecParameters) };
    if result < 0 {
        panic!("failed to copy codec params to codec context");
    }

    let result = unsafe { ffi::avcodec_open2(pCodecContext, pCodec, null_mut()) };
    if result < 0 {
        panic!("failed to open codec through avcodec_open2");
    }

    let pFrame =
        unsafe { ffi::av_frame_alloc().as_mut() }.expect("failed to allocated memory for AVFrame");
    let pPacket = unsafe { ffi::av_packet_alloc().as_mut() }
        .expect("failed to allocated memory for AVPacket");

    let mut how_many_packets_to_process = 8;

    while unsafe { ffi::av_read_frame(pFormatContext, pPacket) } >= 0 {
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
        unsafe { ffi::av_packet_unref(pPacket) };
    }

    println!("releasing all the resources");

    unsafe {
        ffi::avformat_close_input(&mut (pFormatContext as *mut _));
        ffi::av_packet_free(&mut (pPacket as *mut _));
        ffi::av_frame_free(&mut (pFrame as *mut _));
        ffi::avcodec_free_context(&mut (pCodecContext as *mut _));
    }
}

fn decode_packet(
    pPacket: &ffi::AVPacket,
    pCodecContext: &mut ffi::AVCodecContext,
    pFrame: &mut ffi::AVFrame,
) -> Result<(), String> {
    let response = unsafe { ffi::avcodec_send_packet(pCodecContext, pPacket) };

    if response < 0 {
        return Err(String::from("Error while sending a packet to the decoder."));
    }

    while response >= 0 {
        let response = unsafe { ffi::avcodec_receive_frame(pCodecContext, pFrame) };
        if response == AVERROR(ffi::EAGAIN as i32) || response == AVERROR_EOF {
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
                unsafe { ffi::av_get_picture_type_char(pFrame.pict_type) },
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
