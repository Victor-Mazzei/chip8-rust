/// This struct represents the memory of chip-8

use constants::CHIP8_RAM_SIZE;

mod CHIP8_RAM_SIZE;

pub struct Memory {
    // the memory have 4096 bytes
    ram: [u8; CHIP8_RAM_SIZE]
    // stack have array of 16 16-bit values
    stack: [u16; 16],
    // Stack pointer
    sp: u8
}

impl Memory {
    fn new () -> self {
        self{}
    }

    //push value on stack and increment stack pointer
    fn stack_push(&self) {

    }

    //decrement stack pointer and return the stack value
    fn pop_stack(&self) {

    }
}