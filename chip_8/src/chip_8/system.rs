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
    pub(crate) config: Config,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

impl Chip8 {
    /// How many times per second should the timer update.
    pub const FREQUENCY_TIMER_UPDATE: usize = 60;
}

impl Chip8 {
    // TODO(nenikitov): Add configuration parameter here
    pub fn new(config: Config) -> Self {
        Self {
            memory: Memory::default(),
            config,
        }
    }

    /// Reset memory and load a ROM into RAM.
    ///
    /// # Arguments
    ///
    /// * `program` - Program to load.
    pub fn load(&mut self, rom: &[u8]) {
        self.memory.load(rom);
    }

    /// Perform a fetch decode execute cycle.
    /// Should be called at around 500-1000hz.
    ///
    /// # Errors
    ///
    /// If the instruction did not execute correctly.
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
    ///
    /// Should be called at a fixed rate of 60 hz.
    /// The constant is [`Chip8::FREQUENCY_TIMER_UPDATE`]
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

    use eyre::Result;

    #[test]
    fn advance_instruction_works() -> Result<()> {
        let mut c = Chip8::default();

        c.load(&[
            0x61, 0x02, // Load 2 into register 1
            0x71, 0x03, // Add 3 to register 1
        ]);

        assert_eq!(c.memory.v[1], 0);
        assert_eq!(c.memory.pc, Memory::PROGRAM_START);

        c.advance_instruction()?;

        assert_eq!(c.memory.v[1], 2);
        assert_eq!(c.memory.pc, Memory::PROGRAM_START + 2);

        c.advance_instruction()?;

        assert_eq!(c.memory.v[1], 5);
        assert_eq!(c.memory.pc, Memory::PROGRAM_START + 4);

        Ok(())
    }
}
