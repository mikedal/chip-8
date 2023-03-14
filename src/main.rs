extern crate sdl2;

mod audio;
mod chip8;

use audio::SquareWave;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use clap::Parser;

use sdl2::audio::AudioSpecDesired;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Path to the ROM file
    #[clap(value_parser)]
    rom_path: PathBuf,
    // Pixel scale factor
    #[clap(long, value_parser, default_value_t = 6)]
    scale_factor: u32,
}

fn freq_to_period_duration(freq_hertz: u64) -> Duration {
    Duration::from_nanos(1_000_000_000 / freq_hertz)
}

#[test]
fn test_freq_to_period_duration() {
    let freq = 1;
    // 1 Hz
    assert_eq!(freq_to_period_duration(freq), Duration::from_secs(1));
    // 1 MHz
    assert_eq!(freq_to_period_duration(1_000_000), Duration::from_micros(1));
}

fn main() {
    let args = Args::parse();
    let filename = args.rom_path;
    let scale_factor = args.scale_factor;
    let filepath = Path::new(&filename);
    assert!(filepath.is_file());

    let mut chip8 = chip8::chip8::create_chip8();
    chip8.load_rom(filepath);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // audio init
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    let audio_device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        })
        .unwrap();
    let window = video_subsystem
        .window(
            "chip8 emulator",
            chip8::chip8::DISPLAY_WIDTH as u32 * scale_factor,
            chip8::chip8::DISPLAY_HEIGHT as u32 * scale_factor,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let cycle_interval = freq_to_period_duration(chip8::chip8::CYCLE_FREQ);
    let mut sound_playing = false;
    let mut last_tick = Instant::now();

    'running: loop {
        let cycle_start = Instant::now();

        if Instant::now() - last_tick >= chip8::chip8::TICK_INTERVAL {
            chip8.timer_tick();
            last_tick = Instant::now();
        }

        chip8.emulate_cycle();
        if chip8.sound_timer > 0 && !sound_playing {
            audio_device.resume();
            sound_playing = true;
        } else if chip8.sound_timer == 0 && sound_playing {
            audio_device.pause();
            sound_playing = false;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        chip8.key_down(keycode);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(keycode) = keycode {
                        chip8.key_up(keycode);
                    }
                }
                _ => {}
            }
        }
        if chip8.draw {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for i in 0..(chip8::chip8::DISPLAY_WIDTH * chip8::chip8::DISPLAY_HEIGHT) {
                if chip8.gfx[i] {
                    let x = i % chip8::chip8::DISPLAY_WIDTH;
                    let y = i / chip8::chip8::DISPLAY_WIDTH;
                    for subpixel_x in 0..scale_factor {
                        for subpixel_y in 0..scale_factor {
                            canvas
                                .draw_point(Point::new(
                                    (x as u32 * scale_factor + subpixel_x) as i32,
                                    (y as u32 * scale_factor + subpixel_y) as i32,
                                ))
                                .unwrap();
                        }
                    }
                }
            }
            canvas.present();
            chip8.draw = false;
        }

        std::thread::sleep((cycle_start + cycle_interval) - Instant::now())
    }
}
