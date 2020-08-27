//! Port from Original code: https://github.com/leandromoreira/ffmpeg-libav-tutorial/blob/master/0_hello_world.c
use rusty_ffmpeg::ffi;

use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Write,
    ptr, slice,
};

fn main() {
    let filepath: CString = CString::new("./examples/slice/bear.mp4").unwrap();

    println!("initializing all the containers, codecs and protocols.");

    let mut format_context_ptr = unsafe { ffi::avformat_alloc_context() };
    if format_context_ptr.is_null() {
        panic!("ERROR could not allocate memory for Format Context");
    }

    println!(
        "opening the input file ({}) and loading format (container) header",
        filepath.to_str().unwrap()
    );

    if unsafe {
        ffi::avformat_open_input(
            &mut format_context_ptr,
            filepath.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
    } != 0
    {
        panic!("ERROR could not open the file");
    }

    let format_context = unsafe { format_context_ptr.as_mut() }.unwrap();

    let format_name = unsafe { CStr::from_ptr((*format_context.iformat).name) }
        .to_str()
        .unwrap();

    println!(
        "format {}, duration {} us, bit_rate {}",
        format_name, format_context.duration, format_context.bit_rate
    );

    println!("finding stream info from format");

    if unsafe { ffi::avformat_find_stream_info(format_context, ptr::null_mut()) } < 0 {
        panic!("ERROR could not get the stream info");
    }

    let mut codec_ptr: *const ffi::AVCodec = ptr::null_mut();
    let mut codec_parameters_ptr: *const ffi::AVCodecParameters = ptr::null_mut();
    let mut video_stream_index = None;

    let streams = unsafe {
        slice::from_raw_parts(format_context.streams, format_context.nb_streams as usize)
    };

    for (i, stream) in streams
        .iter()
        .map(|stream| unsafe { stream.as_ref() }.unwrap())
        .enumerate()
    {
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

        let local_codec_params = unsafe { stream.codecpar.as_ref() }.unwrap();
        let local_codec =
            unsafe { ffi::avcodec_find_decoder(local_codec_params.codec_id).as_ref() }
                .expect("ERROR unsupported codec!");

        match local_codec_params.codec_type {
            ffi::AVMediaType_AVMEDIA_TYPE_VIDEO => {
                if video_stream_index.is_none() {
                    video_stream_index = Some(i);
                    codec_ptr = local_codec;
                    codec_parameters_ptr = local_codec_params;
                }

                println!(
                    "Video Codec: resolution {} x {}",
                    local_codec_params.width, local_codec_params.height
                );
            }
            ffi::AVMediaType_AVMEDIA_TYPE_AUDIO => {
                println!(
                    "Audio Codec: {} channels, sample rate {}",
                    local_codec_params.channels, local_codec_params.sample_rate
                );
            }
            _ => {}
        };

        let codec_name = unsafe { CStr::from_ptr(local_codec.name) }
            .to_str()
            .unwrap();

        println!(
            "\tCodec {} ID {} bit_rate {}",
            codec_name, local_codec.id, local_codec_params.bit_rate
        );
    }
    let codec_context = unsafe { ffi::avcodec_alloc_context3(codec_ptr).as_mut() }
        .expect("failed to allocated memory for AVCodecContext");

    if unsafe { ffi::avcodec_parameters_to_context(codec_context, codec_parameters_ptr) } < 0 {
        panic!("failed to copy codec params to codec context");
    }

    if unsafe { ffi::avcodec_open2(codec_context, codec_ptr, ptr::null_mut()) } < 0 {
        panic!("failed to open codec through avcodec_open2");
    }

    let frame =
        unsafe { ffi::av_frame_alloc().as_mut() }.expect("failed to allocated memory for AVFrame");
    let packet = unsafe { ffi::av_packet_alloc().as_mut() }
        .expect("failed to allocated memory for AVPacket");

    let mut packets_waiting = 8;

    while unsafe { ffi::av_read_frame(format_context, packet) } >= 0 {
        if video_stream_index == Some(packet.stream_index as usize) {
            println!("AVPacket->pts {}", packet.pts);
            decode_packet(packet, codec_context, frame).unwrap();
            packets_waiting -= 1;
            if packets_waiting <= 0 {
                break;
            }
        }
        unsafe { ffi::av_packet_unref(packet) };
    }

    println!("releasing all the resources");

    unsafe {
        ffi::avformat_close_input(&mut (format_context as *mut _));
        ffi::av_packet_free(&mut (packet as *mut _));
        ffi::av_frame_free(&mut (frame as *mut _));
        ffi::avcodec_free_context(&mut (codec_context as *mut _));
    }
}

fn decode_packet(
    packet: &ffi::AVPacket,
    codec_context: &mut ffi::AVCodecContext,
    frame: &mut ffi::AVFrame,
) -> Result<(), String> {
    let mut response = unsafe { ffi::avcodec_send_packet(codec_context, packet) };

    if response < 0 {
        return Err(String::from("Error while sending a packet to the decoder."));
    }

    while response >= 0 {
        response = unsafe { ffi::avcodec_receive_frame(codec_context, frame) };
        if response == ffi::AVERROR(ffi::EAGAIN) || response == ffi::AVERROR_EOF {
            break;
        } else if response < 0 {
            return Err(String::from(
                "Error while receiving a frame from the decoder.",
            ));
        } else {
            println!(
                "Frame {} (type={}, size={} bytes) pts {} key_frame {} [DTS {}]",
                codec_context.frame_number,
                unsafe { ffi::av_get_picture_type_char(frame.pict_type) },
                frame.pkt_size,
                frame.pts,
                frame.key_frame,
                frame.coded_picture_number
            );

            let frame_filename = format!(
                "./examples/slice/output/frame-{}.pgm",
                codec_context.frame_number
            );
            let width = frame.width as usize;
            let height = frame.height as usize;
            let wrap = frame.linesize[0] as usize;
            let data = unsafe { slice::from_raw_parts(frame.data[0], wrap * height) };
            save_gray_frame(data, wrap, width, height, frame_filename).unwrap();
        }
    }
    Ok(())
}

fn save_gray_frame(
    buf: &[u8],
    wrap: usize,
    xsize: usize,
    ysize: usize,
    filename: String,
) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    let data = format!("P5\n{} {}\n{}\n", xsize, ysize, 255);
    file.write_all(data.as_bytes())?;

    for i in 0..ysize {
        file.write_all(&buf[i * wrap..(i * wrap + xsize)])?;
    }
    Ok(())
}
