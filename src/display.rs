
use sdl2;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels;

use constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT, DISPLAY_SCALE};


/// this struct represents the Display output of the chip-8
pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {

    // this function is responsible creating a new display instance by 
    // - getting the sdl2 context
    // - creating the window
    // - @return canvas;
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("CHIP8 RUST EMULATOR", DISPLAY_WIDTH * DISPLAY_SCALE, DISPLAY_HEIGHT * DISPLAY_SCALE)
        .position_centered()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas()
        .build()
        .unwrap();

        canvas.set_draw_color(pixels::Color:RGB(0,0,0));
        canvas.clear();
        canvas.present();

        return Self {
            canvas: canvas
        }
    }
    
    // this function is responsible for drawing pixels to display
    pub fn draw(&mut self, pixels: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {

        for (y, row) in pixels.iter().enumerate() {
            for (x, &column) in row.iter().enumerate() {
                let x = (x as u32) * DISPLAY_SCALE;
                let y = (y as u32) * DISPLAY_SCALE;

                if column == 0 {
                    pixels::Color::RGB(0,0,0);
                } else {
                    pixels::Color::RGB(0,250,0);
                }

                self.canvas.set_draw_color(column);
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, DISPLAY_SCALE, DISPLAY_SCALE));
            }
        }
        self.canvas.present();
    }
}