#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod app;
mod transcode;

use app::CrocusApp;
use eframe::egui;
use ffmpeg_next as ffmpeg;

fn main() -> eframe::Result {
    ffmpeg::init().unwrap();
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    // rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    eframe::run_native(
        "CrocusApp",
        options,
        Box::new(|_| Ok(Box::<CrocusApp>::default())),
    )
}
