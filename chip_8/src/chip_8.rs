use crate::memory::*;

#[derive(Debug)]
pub struct Chip8 {
    pub memory: Memory,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }

    /// Reset memory and load a ROM into RAM.
    ///
    /// # Arguments
    ///
    /// * `program` - Program to load.
    pub fn load(&mut self, program: &[u8]) {
        self.memory.clear();
        self.memory.ram[PROGRAM_START as usize..][..program.len()].copy_from_slice(program);
    }

    /// Perform a next instruction.
    /// Should be called at around 500 - 1000 hz.
    pub fn advance_instruction(&mut self) {
        self.memory.advance_instruction();
    }

    /// Perform an update of the timer.
    /// Should be called at a fixed rate of 60 hz.
    pub fn advance_timer(&mut self) {
        self.memory.advance_timer();
    }
}
