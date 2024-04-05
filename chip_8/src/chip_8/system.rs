use thiserror::Error;

use crate::instruction::*;

use super::*;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum InstructionError {
    #[error("parse error {0}")]
    ParseError(ParseError),
    #[error("execute error {0}")]
    ExecuteError(ExecuteError),
}

impl From<ParseError> for InstructionError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<ExecuteError> for InstructionError {
    fn from(value: ExecuteError) -> Self {
        Self::ExecuteError(value)
    }
}

pub struct Chip8 {
    pub(crate) memory: Memory,
}

impl Chip8 {
    // TODO(nenikitov): Add configuration parameter here
    pub fn new() -> Self {
        Self {
            memory: Memory::default(),
        }
    }

    /// Reset memory and load a ROM into RAM.
    ///
    /// # Arguments
    ///
    /// * `program` - Program to load.
    pub fn load(&mut self, rom: &[u8]) {
        self.memory.load(rom)
    }

    /// Perform a fetch decode execute cycle.
    /// Should be called at around 500-1000hz.
    pub fn advance_instruction(&mut self) -> Result<(), InstructionError> {
        let opcode = Opcode::from((
            self.memory.ram[self.memory.pc as usize],
            self.memory.ram[self.memory.pc as usize + 1],
        ));
        self.memory.increment_pc();
        self.execute(&Instruction::try_from(opcode)?)?;

        Ok(())
    }

    /// Perform an update of the timer.
    /// Should be called at a fixed rate of 60 hz.
    pub fn advance_timer(&mut self) {
        self.memory.advance_timer();
    }

    /// Access system memory.
    pub fn memory(&self) -> &Memory {
        &self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
