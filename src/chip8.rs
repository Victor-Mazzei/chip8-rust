/// this struct is responsible for binding all the chip8 functionalities 

use memory:Memory;
use cpu:Cpu;

mod memory;
mod processor;

pub struct Chip8 {
    memory: Memory,
    cpu: Cpu
}

impl Chip8 {
    fn new () -> self {
        self {

        }
    }
}