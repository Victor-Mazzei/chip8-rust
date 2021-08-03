/// this struct is responsible for binding all the chip8 functionalities 

use super::{cpu::Cpu, memory::Memory, keyboard::Keyboard};
#[derive(Debug)]
pub struct Chip8 {
    pub memory: Memory,
    pub cpu: Cpu,
    pub keyboard: Keyboard
}

impl Chip8 {
    pub fn new (cpu: Cpu, memory: Memory, keyboard: Keyboard) -> Self {

        Self {
            cpu,
            memory,
            keyboard
        }
    } 

}