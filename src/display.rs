use super::constants::{DISPLAY_WIDTH, DISPLAY_HEIGHT};

// this struct represents the Display output of the chip-8
#[derive(Debug)]
pub struct Display {
    pub pixel: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    pub pixel_collision: bool
}

impl Display {

    // this function is responsible creating a new display instance by 
    // - getting the sdl2 context
    // - creating the window
    // - @return canvas;
    pub fn new() -> Self {

        Self {
            pixel: [[false;DISPLAY_HEIGHT];DISPLAY_WIDTH],
            pixel_collision: false
        }
    }



    pub fn screen_is_set(&self, x:usize,y:usize) -> bool {
        // println!("is set ? x {}, y {}, {}", x, y, self.pixel[x][y]);
        //self.screen_in_bounds(x,y);
        self.pixel[y][x]
        
    }

    pub fn draw_sprite(&mut self, x:usize, y:usize, sprite: &[u8], size_sprite: u8) -> bool {
        // println!("X:{} Y: {}, SPRITE:{:?}, SIZE: {}", x, y, sprite, size_sprite);


        // for ly in 0 .. size_sprite {
            
        //     let c = sprite[ly as usize];
        //     println!("C is {:#03x}", c);
        //     for lx in 0 .. 8 {
        //         //if the current bit == 0 skip to the next
        //         if (c & (0b10000000 >> lx)) == 0 {
        //             // println!("SKIP");
        //             continue;
        //         }

        //         if  self.pixel[(lx as usize + x) & DISPLAY_WIDTH][(ly as usize + y) % DISPLAY_HEIGHT]{
        //             // println!("COLLISION");
        //             self.pixel_collision = true;
        //         }
        //         // println!("Set Pixel");
        //         self.pixel[(lx as usize + x) & DISPLAY_WIDTH as usize][(ly as usize + y) % DISPLAY_HEIGHT as usize] ^= true; 
                
        //     }
            
        // } 
        //println!("{:?}", self.pixel);

        self.pixel_collision
    }
   

}