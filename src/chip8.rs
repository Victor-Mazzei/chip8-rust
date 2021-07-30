/// this struct is responsible for binding all the chip8 functionalities 

use memory:Memory;
use processor:Processor;

mod memory;
mod processor;

pub struct Chip8 {
    memory: Memory,
    processor: Processor
}

impl Chip8 {
    fn new () -> self {
        self {

        }
    }
}