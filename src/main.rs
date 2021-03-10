mod chip8;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

use crate::chip8::chip8::create_chip8;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let filepath = Path::new(filename);
    assert!(filepath.is_file());

    let mut chip8 = create_chip8();
    chip8.load_rom(filepath);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "chip8 emulator",
            chip8::chip8::DISPLAY_WIDTH as u32,
            chip8::chip8::DISPLAY_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        chip8.emulate_cycle();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        if chip8.draw {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            for i in 0..(chip8::chip8::DISPLAY_WIDTH * chip8::chip8::DISPLAY_HEIGHT) {
                if chip8.gfx[i] {
                    canvas
                        .draw_point(Point::new(
                            i as i32 % chip8::chip8::DISPLAY_WIDTH as i32,
                            i as i32 / chip8::chip8::DISPLAY_WIDTH as i32,
                        ))
                        .unwrap();
                }
            }
            canvas.present();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
