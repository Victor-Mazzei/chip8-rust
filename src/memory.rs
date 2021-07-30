/// This struct represents the memory of chip-8

pub struct Memory {
    // the memory have 4096 bytes
    ram: [u8;4096]
    // stack have array of 16 16-bit values
    stack: [u16;16],
    // Stack pointer
    sp: u8
}

impl Memory {
    fn new () -> self {
        self{}
    }
}