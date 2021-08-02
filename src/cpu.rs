use std::{thread, time};

/// this struct represents the CPU of chip-8
#[derive(Debug)]
pub struct Cpu {
    //Program counter
    pub pc: usize,
    //General purpose register
    pub v: [u8;16],
    //I register
    pub i: usize,
    //Stack pointer,
    pub sp: usize,
    //Delay timer
    pub delay_timer: u8,
    //Sound timer
    pub sound_timer: u8,

}

impl Cpu {
    pub fn new() -> Self {
        //initializing all the registers with 0
        Self {
            pc: 0,
            v:[0; 16],
            i:0,
            sp:0,
            delay_timer:0,
            sound_timer:0
        }
    }

    pub fn check_delay_timer(&mut self) {
        
        if self.delay_timer > 0 {
            thread::sleep(time::Duration::from_millis(100));
            self.delay_timer -=1;
            println!("DELAY _TIMER");
        }
    }

    pub fn check_sound_timer(&mut self) {
        
        if self.sound_timer > 0 {
           
            println!("BEEP!!!!!");
            self.sound_timer -=1;
        }
    }

}