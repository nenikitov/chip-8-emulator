use crate::{instructions::*, memory::*};

#[derive(Debug)]
pub struct Processor {
    pub memory: Memory,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        instruction.execute(&mut self.memory);
    }

    pub fn fetch(&mut self) -> Instruction {
        let opcode = Opcode::from((
            self.memory.ram[self.memory.program_counter as usize],
            self.memory.ram[self.memory.program_counter as usize + 1]
        ));
        self.memory.program_counter += 2;
        Instruction::from(opcode)
    }
}

