use crate::{memory::*, instruction::*};

#[derive(Debug)]
pub struct Chip8 {
    memory: Memory
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new()
        }
    }

    pub fn load(&mut self, program: &[u8]) {
        self.memory.clear();
        self.memory.ram[PROGRAM_START as usize..PROGRAM_START as usize + program.len()].copy_from_slice(program);
    }

    pub fn advance(&mut self) {
        let opcode = Opcode::from((
            self.memory.ram[self.memory.program_counter as usize],
            self.memory.ram[self.memory.program_counter as usize + 1]
        ));
        self.memory.program_counter += 2;
        Instruction::from(opcode).execute(&mut self.memory);
    }

    pub fn display(&self) -> Box<[&[bool]]> {
        self.memory.display
            .chunks(SIZE_DISPLAY.0 as usize)
            .map(|c| c.as_ref())
            .collect()
    }
}
