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
    pub fn new () -> Self {
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
}