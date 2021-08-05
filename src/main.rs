extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{thread, time};

use constants::{DISPLAY_HEIGHT,DISPLAY_WIDTH,DISPLAY_SCALE};
use chip8::Chip8;
use cpu::Cpu;
use memory::Memory;
use keyboard::Keyboard;
use display::Display;


mod constants;
mod chip8;
mod cpu;
mod memory;
mod keyboard;
mod display;


fn main() {
    //
    let mut chip8 = Chip8::new(Cpu::new(),Memory::new(),Keyboard::new(), Display::new());


    //println!("{:?}", chip8);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("CHIP-8 Rust", DISPLAY_WIDTH * DISPLAY_SCALE, DISPLAY_HEIGHT * DISPLAY_SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        thread::sleep(time::Duration::from_millis(100));
    }
    

    

}
