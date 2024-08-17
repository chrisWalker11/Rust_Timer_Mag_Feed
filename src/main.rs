#![windows_subsystem = "windows"]

use eframe::egui;
use rodio::{OutputStream, Sink};
use std::time::{Duration, Instant};

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Timer with Sound", options, Box::new(|_cc| Box::new(MyApp::default())));
}

struct MyApp {
    start_timer: bool,
    time_remaining: Option<u32>,  // Time in seconds
    flashing: bool,
    last_flash_time: Instant,
    flash_on: bool,
    last_update_time: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            start_timer: false,
            time_remaining: None,
            flashing: false,
            last_flash_time: Instant::now(),
            flash_on: false,
            last_update_time: Instant::now(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();

        if self.flashing {
            if now.duration_since(self.last_flash_time) > Duration::from_millis(500) {
                self.flash_on = !self.flash_on;
                self.last_flash_time = now;
                ctx.request_repaint(); // Request a repaint to create the flashing effect
            }
        } else if self.start_timer {
            if now.duration_since(self.last_update_time) >= Duration::from_secs(1) {
                if let Some(time_remaining) = &mut self.time_remaining {
                    if *time_remaining > 0 {
                        *time_remaining -= 1;
                    }
                    self.last_update_time = now;

                    if *time_remaining == 0 {
                        self.start_timer = false;
                        self.flashing = true;
                        play_sound();
                    }
                }
            }
            ctx.request_repaint(); // Continue requesting repaint to keep the timer going
        }

        egui::CentralPanel::default()
            .frame(if self.flashing && self.flash_on {
                egui::Frame::central_panel(&ctx.style()).fill(egui::Color32::RED)
            } else {
                egui::Frame::central_panel(&ctx.style())
            })
            .show(ctx, |ui| {
                if self.flashing {
                    ui.heading("Time's Up!");
                    ui.label("The timer has finished.");
                    if ui.button("Reset Timer").clicked() {
                        self.flashing = false;
                        self.start_timer = false;
                        self.time_remaining = None;
                    }
                } else {
                    ui.heading("Fixed Timer: 3m 15s");

                    if ui.button("Start Timer").clicked() {
                        self.start_timer = true;
                        self.time_remaining = Some(3 * 60 + 15); // 3 minutes and 15 seconds
                        self.last_update_time = Instant::now();
                    }

                    if let Some(time_remaining) = self.time_remaining {
                        let minutes = time_remaining / 60;
                        let seconds = time_remaining % 60;
                        ui.label(format!("Time remaining: {}m {}s", minutes, seconds));
                    }
                }
            });
    }
}

fn play_sound() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    // Embed the alarm.wav file directly into the binary
    let file = std::io::Cursor::new(include_bytes!("alarm.wav").as_ref());
    let source = rodio::Decoder::new(file).unwrap();
    
    sink.append(source);
    sink.sleep_until_end();
}

