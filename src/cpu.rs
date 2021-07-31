/// this struct represents the CPU of chip-8

pub struct Cpu {
    //Program counter
    pc: u16,
    //General purpose register
    v: [u8;16]
    //Delay timer
    delay_timer: u8,
    //Sound timer
    sound_timer: u8,

}

impl Cpu {
    fn new () -> self {
        self {

        }
    }
}