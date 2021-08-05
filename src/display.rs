use super::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

// this struct represents the Display output of the chip-8
#[derive(Debug)]
pub struct Display {
    pub pixel: [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize],
    pub pixel_collision: bool
}

impl Display {

    // this function is responsible creating a new display instance by 
    // - getting the sdl2 context
    // - creating the window
    // - @return canvas;
    pub fn new() -> Self {

        Self {
            pixel: [[false;DISPLAY_WIDTH as usize];DISPLAY_HEIGHT as usize],
            pixel_collision: false
        }
    }

    fn screen_set(&mut self, x:usize, y:usize) {
        self.pixel[y][x] = true
    }

    fn screen_is_set(&self, x:usize,y:usize) -> bool {
        self.pixel[y][x]
    }

    fn draw_sprite(&mut self, x:usize, y:usize, sprite: Vec<u8>, size_sprite: u8) -> bool {
        
        for ly in 0..size_sprite {
            
            let c = sprite[ly as usize];

            for lx in 0..8 {
                //if the current bit == 0 skip to the next
                if (c & (0b10000000 >> lx)) == 0 {
                    continue;
                }
                self.pixel[(ly as usize + y) % DISPLAY_HEIGHT as usize][(lx as usize + x) & DISPLAY_WIDTH as usize] ^= true;  
            }
        } 

        self.pixel_collision
    }
   

}