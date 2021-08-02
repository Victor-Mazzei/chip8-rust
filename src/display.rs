
use sdl2;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels;


const DISPLAY_WIDTH: u32 = 64;
const DISPLAY_HEIGHT: u32 = 32;
const DISPLAY_SCALE: u32 = 10;


// this struct represents the Display output of the chip-8
#[derive(Debug)]
pub struct Display {
    pub canvas: Canvas<Window>,
}

impl Display {

    // this function is responsible creating a new display instance by 
    // - getting the sdl2 context
    // - creating the window
    // - @return canvas;
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("CHIP8 RUST EMULATOR", 64 * 20, 32 * 20)
        .position_centered()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas()
        .build()
        .unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        return Self {
            canvas: canvas
        }
    }
    
    // this function is responsible for drawing pixels to display
    pub fn draw(&mut self, pixels: &[[u8; 64]; 32]) {

        for (y, row) in pixels.iter().enumerate() {
            for (x, &column) in row.iter().enumerate() {
                let x = (x as i32) * 20;
                let y = (y as i32) * 20;

                if column == 0 {
                    pixels::Color::RGB(0,0,0);
                } else {
                    pixels::Color::RGB(0,250,0);
                }

                self.canvas.set_draw_color(column);
                let _ = self.canvas
                    .fill_rect(Rect::new(x , y, 20, 20));
            }
        }
        self.canvas.present();
    }
}