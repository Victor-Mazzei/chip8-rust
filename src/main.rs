use chip8::Chip8;
use cpu::Cpu;
use memory::Memory;

mod chip8;
mod cpu;
mod memory;

fn main() {
    //
    let mut chip8 = Chip8::new(Memory::new(), Cpu::new());

    println!("{} SP BEFORE PUSH", chip8.cpu.sp);

    //println!("{:?}, CHIP 8 MEMORY", chip8.memory);

    chip8.memory.stack_push(&mut chip8.cpu, 0xf1);

    println!("{:?}", chip8.memory.stack);

    println!("{} SP AFTER PUSH", chip8.cpu.sp);

    //println!("{} value push on stack", chip8.memory.stack[chip8.cpu.sp - 1]);

    

}
