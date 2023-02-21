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
        todo!()
    }

    pub fn fetch(&mut self) -> Instruction {
        let instruction_bytes = self.read_instruction_bytes();
        todo!()
    }

    fn read_instruction_bytes(&mut self) -> u16 {
        let (byte_1, byte_2) = (
            self.memory.ram[self.memory.program_couter as usize],
            self.memory.ram[self.memory.program_couter as usize + 1],
        );
        self.memory.program_couter += 1;

        ((byte_1 as u16) << 8) | (byte_2 as u16)
    }
}
