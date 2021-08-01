/// this struct is responsible for binding all the chip8 functionalities 

use super::{cpu::Cpu, memory::Memory};
#[derive(Debug)]
pub struct Chip8 {
    pub memory: Memory,
    pub cpu: Cpu
}

impl Chip8 {
    pub fn new (memory: Memory, cpu: Cpu) -> Self {

        Self {
            memory: memory,
            cpu: cpu
        }
    } 

}