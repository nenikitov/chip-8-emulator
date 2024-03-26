use crate::{instruction::*, memory::*};

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

    /// Reset memory and load a ROM into RAM>
    ///
    /// # Arguments
    ///
    /// * `program` - Program to load.
    pub fn load(&mut self, program: &[u8]) {
        self.memory.clear();
        self.memory.ram[PROGRAM_START as usize..PROGRAM_START as usize + program.len()]
            .copy_from_slice(program);
    }

    /// Perform a next instruction.
    pub fn advance(&mut self) {
        let opcode = Opcode::from((
            self.memory.ram[self.memory.pc as usize],
            self.memory.ram[self.memory.pc as usize + 1],
        ));
        self.memory.pc += 2;
        Instruction::from(opcode).execute(&mut self.memory);
    }
}
