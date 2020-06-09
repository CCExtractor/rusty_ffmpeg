# Rusty FFmpeg

FFI binding for FFmpeg inner library.

Run `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo build` to build it(where `PKG_CONFIG_PATH` points to `*.pc` files). Then you can use `cargo test` to test the generated bindings.

Or you can `PKG_CONFIG_PATH="$HOME/ffmpeg_build/lib/pkgconfig" cargo test` directly.

```bash
running 120 tests
test bindgen_test_layout_AVABufferSinkParams ... ok
test bindgen_test_layout_AVBPrint ... ok
test bindgen_test_layout_AVBSFContext ... ok
test bindgen_test_layout_AVBitStreamFilterContext ... ok
test bindgen_test_layout_AVBlowfish ... ok
test bindgen_test_layout_AVBufferSinkParams ... ok
test bindgen_test_layout_AVBufferRef ... ok
test bindgen_test_layout_AVBitStreamFilter ... ok
test bindgen_test_layout_AVCPBProperties ... ok
test bindgen_test_layout_AVChapter ... ok
test bindgen_test_layout_AVClass ... ok
test bindgen_test_layout_AVBufferSrcParameters ... ok
test bindgen_test_layout_AVCodecDescriptor ... ok
test bindgen_test_layout_AVCodecHWConfig ... ok
test bindgen_test_layout_AVCodec ... ok
test bindgen_test_layout_AVCodecParser ... ok
test bindgen_test_layout_AVCodecParserContext ... ok
test bindgen_test_layout_AVCodecContext ... ok
test bindgen_test_layout_AVDVProfile ... ok
test bindgen_test_layout_AVCodecParameters ... ok
test bindgen_test_layout_AVComponentDescriptor ... ok
test bindgen_test_layout_AVDeviceInfoList ... ok
test bindgen_test_layout_AVDeviceInfo ... ok
test bindgen_test_layout_AVDeviceCapabilitiesQuery ... ok
test bindgen_test_layout_AVDeviceRect ... ok
test bindgen_test_layout_AVDictionaryEntry ... ok
test bindgen_test_layout_AVFilter ... ok
test bindgen_test_layout_AVDownmixInfo ... ok
test bindgen_test_layout_AVFifoBuffer ... ok
test bindgen_test_layout_AVFilterGraph ... ok
test bindgen_test_layout_AVFilterInOut ... ok
test bindgen_test_layout_AVFilterContext ... ok
test bindgen_test_layout_AVFrame ... ok
test bindgen_test_layout_AVFrameSideData ... ok
test bindgen_test_layout_AVHWDeviceContext ... ok
test bindgen_test_layout_AVHWAccel ... ok
test bindgen_test_layout_AVHWFramesConstraints ... ok
test bindgen_test_layout_AVHWFramesContext ... ok
test bindgen_test_layout_AVIODirContext ... ok
test bindgen_test_layout_AVIOContext ... ok
test bindgen_test_layout_AVIODirEntry ... ok
test bindgen_test_layout_AVIOInterruptCB ... ok
test bindgen_test_layout_AVFilterLink ... ok
test bindgen_test_layout_AVIndexEntry ... ok
test bindgen_test_layout_AVInputFormat ... ok
test bindgen_test_layout_AVLFG ... ok
test bindgen_test_layout_AVOptionRange ... ok
test bindgen_test_layout_AVMotionVector ... ok
test bindgen_test_layout_AVOption ... ok
test bindgen_test_layout_AVOptionRanges ... ok
test bindgen_test_layout_AVFormatContext ... ok
test bindgen_test_layout_AVPacket ... ok
test bindgen_test_layout_AVPacketList ... ok
test bindgen_test_layout_AVPacketSideData ... ok
test bindgen_test_layout_AVOption__bindgen_ty_1 ... ok
test bindgen_test_layout_AVPanScan ... ok
test bindgen_test_layout_AVPixFmtDescriptor ... ok
test bindgen_test_layout_AVProbeData ... ok
test bindgen_test_layout_AVProducerReferenceTime ... ok
test bindgen_test_layout_AVProfile ... ok
test bindgen_test_layout_AVProgram ... ok
test bindgen_test_layout_AVRational ... ok
test bindgen_test_layout_AVRegionOfInterest ... ok
test bindgen_test_layout_AVReplayGain ... ok
test bindgen_test_layout_AVPicture ... ok
test bindgen_test_layout_AVStereo3D ... ok
test bindgen_test_layout_AVStream__bindgen_ty_1 ... ok
test bindgen_test_layout_AVStream ... ok
test bindgen_test_layout_AVSubtitleRect ... ok
test bindgen_test_layout_AVSubtitle ... ok
test bindgen_test_layout_AVXTEA ... ok
test bindgen_test_layout_AVTimecode ... ok
test bindgen_test_layout_RcOverride ... ok
test bindgen_test_layout_FFTComplex ... ok
test bindgen_test_layout_AVOutputFormat ... ok
test bindgen_test_layout_SwsFilter ... ok
test bindgen_test_layout_SwsVector ... ok
test bindgen_test_layout__G_fpos64_t ... ok
test bindgen_test_layout__G_fpos_t ... ok
test bindgen_test_layout___fsid_t ... ok
test bindgen_test_layout___mbstate_t ... ok
test bindgen_test_layout__IO_marker ... ok
test bindgen_test_layout___locale_struct ... ok
test bindgen_test_layout___mbstate_t__bindgen_ty_1 ... ok
test bindgen_test_layout__IO_FILE ... ok
test bindgen_test_layout___pthread_cond_s__bindgen_ty_1 ... ok
test bindgen_test_layout___pthread_cond_s ... ok
test bindgen_test_layout___pthread_cond_s__bindgen_ty_2 ... ok
test bindgen_test_layout___pthread_cond_s__bindgen_ty_2__bindgen_ty_1 ... ok
test bindgen_test_layout___pthread_mutex_s ... ok
test bindgen_test_layout___pthread_rwlock_arch_t ... ok
test bindgen_test_layout___pthread_cond_s__bindgen_ty_1__bindgen_ty_1 ... ok
test bindgen_test_layout___pthread_internal_list ... ok
test bindgen_test_layout_av_intfloat32 ... ok
test bindgen_test_layout___sigset_t ... ok
test bindgen_test_layout_av_intfloat64 ... ok
test bindgen_test_layout_div_t ... ok
test bindgen_test_layout_drand48_data ... ok
test bindgen_test_layout_imaxdiv_t ... ok
test bindgen_test_layout___va_list_tag ... ok
test bindgen_test_layout_fd_set ... ok
test bindgen_test_layout_ldiv_t ... ok
test bindgen_test_layout_lldiv_t ... ok
test bindgen_test_layout_ff_pad_helper_AVBPrint ... ok
test bindgen_test_layout_itimerspec ... ok
test bindgen_test_layout_pthread_attr_t ... ok
test bindgen_test_layout_max_align_t ... ok
test bindgen_test_layout_pthread_barrier_t ... ok
test bindgen_test_layout_pthread_cond_t ... ok
test bindgen_test_layout_pthread_condattr_t ... ok
test bindgen_test_layout_pthread_mutex_t ... ok
test bindgen_test_layout_pthread_mutexattr_t ... ok
test bindgen_test_layout_pthread_rwlock_t ... ok
test bindgen_test_layout_random_data ... ok
test bindgen_test_layout_timespec ... ok
test bindgen_test_layout_tm ... ok
test bindgen_test_layout_timeval ... ok
test bindgen_test_layout_vaapi_context ... ok
test bindgen_test_layout_pthread_barrierattr_t ... ok
test bindgen_test_layout_pthread_rwlockattr_t ... ok

test result: ok. 120 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
