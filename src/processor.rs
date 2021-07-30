/// this struct represents the processor of chip-8

pub struct Processor {
    //Program counter
    pc: u16,
    //Stack counter
    sp: u8,
    //General purpose register
    v:[u8;16]
    //Delay timer
    delay_timer: u8,
    //Sound timer
    sound_timer: u8,

}

impl Processor {
    fn new () -> self {
        self {

        }
    }
}