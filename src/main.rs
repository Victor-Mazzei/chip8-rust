extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::{thread, time};

use chip8::Chip8;
use constants::{DISPLAY_HEIGHT, DISPLAY_SCALE, DISPLAY_WIDTH};
use cpu::Cpu;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

mod chip8;
mod constants;
mod cpu;
mod display;
mod keyboard;
mod memory;

fn main() {
    //
    let mut chip8 = Chip8::new(Cpu::new(), Memory::new(), Keyboard::new(), Display::new());

    //println!("{:?}", chip8);
    chip8.initialize();
    chip8.display.draw_sprite(0, 0, &chip8.memory.ram, 5);

    //println!("{:?}", chip8);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "CHIP-8 Rust",
            DISPLAY_WIDTH as u32 * DISPLAY_SCALE,
            DISPLAY_HEIGHT as u32 * DISPLAY_SCALE,
        )
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
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
        // Todo fix bug width..
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                if chip8.display.screen_is_set(x, y) {
                    canvas.set_draw_color(Color::WHITE);
                    // canvas.clear();
                    let _ = canvas.fill_rect(Rect::new(
                        x as i32 * DISPLAY_SCALE as i32,
                        y as i32 * DISPLAY_SCALE as i32,
                        DISPLAY_SCALE,
                        DISPLAY_SCALE,
                    ));
                }
            }
        }
        // canvas.clear();
        canvas.present();
        // The rest of the game loop goes here...

        // canvas.present();
        thread::sleep(time::Duration::from_millis(100));
    }
}
