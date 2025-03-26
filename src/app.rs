use std::rc::Rc;
use std::sync::{Arc, Mutex};

use bimap::BiMap;

use eframe::egui;
use egui::{ColorImage, TextureHandle};

use rayon::prelude::*;

use ffmpeg_next as ffmpeg;

use ffmpeg::frame::Video as VideoFrame;
use ffmpeg::software::scaling::{context::Context as AvScaler, flag::Flags as AvScalerFlags};
use ffmpeg::dictionary as FFDict;
use lru::LruCache;

// const VIDEO_PATH: &'static str = &"vendor/vids/ETS2-record.mp4";
const VIDEO_PATH: &'static str = &"https://www.w3schools.com/html/mov_bbb.mp4";
const BUFFER_SIZE: u32 = 128 * 1024 * 1024; // 128MB, TODO: make it configurable

fn trunk_by_indices<'a, T>(vec: &'a [T], idx: &[usize]) -> Vec<&'a [T]> {
    let mut result = Vec::new();
    let mut start = 0;

    for &i in idx {
        if i <= vec.len() {
            result.push(&vec[start..i]);
            start = i;
        }
    }

    // Push the remaining elements after the last index
    if start < vec.len() {
        result.push(&vec[start..]);
    }

    result
}

pub(crate) struct CrocusApp {
    frame_idx: usize,
    buffer: LruCache<usize, Rc<VideoFrame>>,
    frame_ts_map: BiMap<usize, i64>,
    canvas: Option<TextureHandle>, // the field to display the video
    scaler: AvScaler,
    frame_len: usize,

    ictx: ffmpeg::format::context::Input,
    decoder: ffmpeg::codec::decoder::Video,
    video_stream_index: usize,
}

impl CrocusApp {
    fn new() -> Result<Self, ffmpeg::Error> {
        // let mut ictx = ffmpeg::format::input(&VIDEO_PATH)?;
        let mut ictx = Self::open_input_with_hwaccel(VIDEO_PATH)?;
        let input = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let decoder = context.decoder().video()?;
        let scaler = AvScaler::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg::format::Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            AvScalerFlags::BICUBIC,
        )?;

        let buffer_len = (BUFFER_SIZE / (decoder.width() * decoder.height() * 3)) as usize;
        let buffer = LruCache::new(std::num::NonZeroUsize::new(buffer_len).unwrap());

        let timestamps = Vec::new();
        let timestamps_mutex = Arc::new(Mutex::new(timestamps));

        let packets = ictx
            .packets()
            .filter(|(s, _p)| s.index() == video_stream_index)
            .map(|(_s, p)| p)
            .collect::<Vec<_>>();

        let key_packets = packets
            .iter()
            .enumerate()
            .filter(|(_i, p)| p.is_key())
            .map(|(i, _p)| i)
            .collect::<Vec<_>>();

        // trunk the packets by key frames
        let packets = trunk_by_indices(&packets, &key_packets);
        
        // parallel decode the packets and collect the timestamps
        // TODO: make it async & lazy to avoid long freeze in large videoes
        packets.par_iter().for_each(|packet| {
            let ictx = ffmpeg::format::input(&VIDEO_PATH).unwrap();
            let input = ictx.stream(video_stream_index).unwrap();
            let context =
                ffmpeg::codec::context::Context::from_parameters(input.parameters()).unwrap();
            let mut decoder = context.decoder().video().unwrap();
            let mut frame = VideoFrame::empty();
            for p in packet.iter() {
                decoder.send_packet(p).unwrap();
                while decoder.receive_frame(&mut frame).is_ok() {
                    let ts = frame.timestamp().unwrap();
                    timestamps_mutex.lock().unwrap().push(ts);
                }
            }
        });

        timestamps_mutex.lock().unwrap().sort();

        let frame_ts_map = timestamps_mutex
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, ts)| (i, *ts))
            .collect::<BiMap<_, _>>();

        let frame_len = frame_ts_map.len();

        Ok(Self {
            frame_idx: 0,
            buffer,
            frame_ts_map,
            canvas: None,
            scaler,
            frame_len,
            ictx,
            decoder,
            video_stream_index,
        })
    }

    fn open_input_with_hwaccel(video_path: &str) -> Result<ffmpeg::format::context::Input, ffmpeg::Error> {
        let mut options = FFDict::Owned::new();
        #[cfg(target_os = "linux")]
        options.set("hwaccel", "vaapi");
        #[cfg(target_os = "windows")]
        options.set("hwaccel", "dxva2");

        let input = ffmpeg::format::input_with_dictionary(video_path, options)?;
        Ok(input)
    }

    fn frame2img(&mut self, frame: &ffmpeg::frame::Video) -> Result<ColorImage, ffmpeg::Error> {
        let mut frame_scaled = ffmpeg::frame::Video::empty();
        self.scaler.run(frame, &mut frame_scaled)?;
        Ok(ColorImage::from_rgb(
            [frame.width() as _, frame.height() as _],
            &frame_scaled.data(0),
        ))
    }

    fn update_texture(&mut self, ctx: &egui::Context) -> Result<(), ffmpeg::Error> {
        let target = self.frame_idx;
        if let Some(frame) = self.buffer.get(&target).cloned() {
            println!("Cache hit, frame idx: {}", target);
            let texture = self.frame2img(&frame)?;
            self.canvas =
                Some(ctx.load_texture("video-frame", texture, egui::TextureOptions::default()));
            Ok(())
        } else {
            let target_ts = self.frame_ts_map.get_by_left(&target);
            let target_ts = match target_ts {
                Some(ts) => {
                    println!("seek to ts: {}, frame: {}", ts, target);
                    ts
                }
                None => {
                    println!("No ts for frame: {}", target);
                    return Ok(());
                }
            };
            unsafe {
                ffmpeg::ffi::av_seek_frame(
                    self.ictx.as_mut_ptr(),
                    self.video_stream_index as i32,
                    *target_ts,
                    ffmpeg::ffi::AVSEEK_FLAG_BACKWARD,
                );
            }
            let mut frame = ffmpeg::frame::Video::empty();
            let mut flag = false;
            for (stream, packet) in self.ictx.packets() {
                if stream.index() == self.video_stream_index {
                    self.decoder.send_packet(&packet)?;
                    while self.decoder.receive_frame(&mut frame).is_ok() {
                        let ts = frame.timestamp().unwrap();
                        let frame_idx = self.frame_ts_map.get_by_right(&ts);
                        let frame_idx = match frame_idx {
                            Some(frame_idx) => *frame_idx,
                            None => {
                                println!("No frame idx for ts: {}", ts);
                                0
                            }
                        };
                        self.buffer.put(frame_idx, Rc::new(frame.clone()));
                        if frame.timestamp().unwrap() == *target_ts {
                            flag = true;
                            break;
                        }
                    }
                }
                if flag {
                    break;
                }
            }
            let texture = self.frame2img(&frame)?;
            self.canvas =
                Some(ctx.load_texture("video-frame", texture, egui::TextureOptions::default()));
            Ok(())
        }
    }
}

impl Default for CrocusApp {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl eframe::App for CrocusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // the slide to seek the video
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width();
            ui.spacing_mut().slider_width = width - 100.0;
            ui.horizontal(|ui| {
                if ui
                    .add_sized(
                        ui.available_size(),
                        egui::Slider::new(&mut self.frame_idx, 0..=(self.frame_len - 1))
                            .drag_value_speed(0.5)
                            .step_by(1.),
                    )
                    .changed()
                {
                    self.update_texture(ctx).unwrap();
                }
            });

            if let Some(canvas) = &self.canvas {
                ui.image(canvas);
            }
        });
    }
}
