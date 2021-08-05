
static DISPLAY_WIDTH = 64;
static DISPLAY_HEIGHT = 32;
static DISPLAY_SCALE = 10;


// this struct represents the Display output of the chip-8
#[derive(Debug)]
pub struct Display {

    pub pixel: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]
}

impl Display {

    // this function is responsible creating a new display instance by 
    // - getting the sdl2 context
    // - creating the window
    // - @return canvas;
    pub fn new() -> Self {

        Self {
            pixel: [[false;DISPLAY_WIDTH];DISPLAY_HEIGHT]
        }
    }

    fn screen_set(&mut self, x:u8, y:u8) {
        self.pixel[y][x] = true
    }

    fn screen_is_set(&self, x:u8,y:u8) -> bool {
        self.pixel[y][x]
    }
   

}