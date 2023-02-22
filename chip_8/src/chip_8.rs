use crate::{memory::*, processor::*};

#[derive(Debug)]
pub struct Chip8 {
    processor: Processor,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            processor: Processor::new()
        }
    }

    pub fn load(&mut self, program: &[u8]) {
        self.processor.memory.clear();
        self.processor.memory.ram[PROGRAM_START as usize..PROGRAM_START as usize + program.len()].copy_from_slice(program);
    }

    pub fn advance(&mut self) {
        let instruction = self.processor.fetch();
        self.processor.execute(instruction);
    }

    pub fn display_size(&self) -> (u16, u16) {
        SIZE_DISPLAY
    }

    pub fn display(&self) -> &[bool; SIZE_DISPLAY_TOTAL] {
        &self.processor.memory.display
    }
}

