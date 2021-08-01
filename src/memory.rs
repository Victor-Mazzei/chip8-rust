/// This struct represents the memory of chip-8
//use constants::{CHIP8_RAM_SIZE,CHIP8_STACK_DEPTH};
use crate::Chip8;
use crate::Cpu;

const CHIP8_RAM_SIZE: usize = 4096;
const CHIP8_STACK_DEPTH: usize = 16;

#[derive(Debug)]
pub struct Memory {
    // the memory have 4096 bytes
    pub ram: [u8; CHIP8_RAM_SIZE],
    // stack have array of 16 16-bit values
    pub stack: [u16; CHIP8_STACK_DEPTH],
}

impl Memory {
    pub fn new () -> Self {
        Self {
            ram:[0; CHIP8_RAM_SIZE],
            stack:[0; CHIP8_STACK_DEPTH]
        }
    }

    fn check_memory_bounds(&self, index: usize) {
        assert!(index > 0 && index < CHIP8_RAM_SIZE);
    }

    fn check_stack_bounds(&self, sp: usize) {
        assert!(sp < CHIP8_STACK_DEPTH);
    }


    pub fn memory_set_value(&mut self, index: usize, value: u8) {
        self.check_memory_bounds(index);
        self.ram[index] = value;
    }

    pub fn memory_get_value(&self, index:usize) -> u8 {
        self.check_memory_bounds(index);
        return self.ram[index];
    }

    pub fn stack_push(&mut self, cpu: &mut Cpu, value: u16) {
        self.check_stack_bounds(cpu.sp);
        self.stack[cpu.sp] = value;
        cpu.sp+= 1;
    }

    pub fn stack_pop(&self, cpu: &mut Cpu) -> u16 {
        cpu.sp-= 1;
        self.check_stack_bounds(cpu.sp);
        return self.stack[cpu.sp];
    }
}