use super::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

// this struct represents the Display output of the chip-8
#[derive(Debug)]
pub struct Display {
    pub pixel: [[bool; DISPLAY_WIDTH as usize]; DISPLAY_HEIGHT as usize]
}

impl Display {

    // this function is responsible creating a new display instance by 
    // - getting the sdl2 context
    // - creating the window
    // - @return canvas;
    pub fn new() -> Self {

        Self {
            pixel: [[false;DISPLAY_WIDTH as usize];DISPLAY_HEIGHT as usize]
        }
    }

    fn screen_set(&mut self, x:usize, y:usize) {
        self.pixel[y][x] = true
    }

    fn screen_is_set(&self, x:usize,y:usize) -> bool {
        self.pixel[y][x]
    }
   

}