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

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum State {
    #[default]
    Ready,
    WaitingForKey {
        vx: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chip8 {
    pub(crate) config: Config,
    pub(crate) memory: Memory,
    pub(crate) state: State,
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
            config,
            memory: Memory::default(),
            state: State::default(),
        }
    }

    /// Access system memory.
    pub fn memory(&self) -> &Memory {
        &self.memory
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
    /// Return an [`InstructionError`] if the instruction did not execute correctly.
    pub fn advance_instruction(&mut self) -> Result<(), InstructionError> {
        if self.state == State::Ready && self.memory.dt == 0 {
            let opcode = Opcode::from((
                self.memory.ram[self.memory.pc as usize],
                self.memory.ram[self.memory.pc as usize + 1],
            ));
            self.memory.increment_pc();
            self.execute(&Instruction::try_from(opcode)?)?;
        }

        Ok(())
    }

    /// Perform an update of the timer.
    ///
    /// Should be called at a fixed rate of 60 hz.
    /// The constant is [`Chip8::FREQUENCY_TIMER_UPDATE`]
    pub fn advance_timer(&mut self) {
        self.memory.advance_timer();
    }

    /// Presses a key by the index.
    ///
    /// # Arguments
    ///
    /// * `key` - The index of the key. Must be between 0x0 and 0xF (inclusive).
    ///
    /// # Errors
    ///
    /// Returns an [`InstructionError`] if the provided key is outside the valid range.
    pub fn press_key(&mut self, key: u8) -> Result<(), InstructionError> {
        if key > 0xF {
            return Err(ExecuteError::InvalidKey(key).into());
        }

        self.memory.keys[key as usize] = true;

        Ok(())
    }

    /// Unpress a key by the index.
    /// Also unblocks the execution if the system was waiting for a key press.
    ///
    /// # Arguments
    ///
    /// * `key` - The index of the key. Must be between 0x0 and 0xF (inclusive).
    ///
    /// # Errors
    ///
    /// Returns an [`InstructionError`] if the provided key is outside the valid range.
    pub fn unpress_key(&mut self, key: u8) -> Result<(), InstructionError> {
        if key > 0xF {
            return Err(ExecuteError::InvalidKey(key).into());
        }

        self.memory.keys[key as usize] = false;

        if let State::WaitingForKey { vx } = self.state {
            self.memory.v[vx] = key;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use eyre::Result;
    use similar_asserts::assert_eq;

    // TODO(nenikitov): Use `rstest` here.

    #[test]
    fn advance_instruction_ready() -> Result<()> {
        let mut c = Chip8::default();

        c.load(&[
            0x61, 0x02, // Load 2 into register 1
            0x71, 0x03, // Add 3 to register 1
        ]);

        assert_eq!(c.memory.v[1], 0);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START);

        c.advance_instruction()?;

        assert_eq!(c.memory.v[1], 2);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START + 2);

        c.advance_instruction()?;

        assert_eq!(c.memory.v[1], 5);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START + 4);

        Ok(())
    }

    #[test]
    fn advance_instruction_waiting_for_key() -> Result<()> {
        let mut c = Chip8::default();

        c.state = State::WaitingForKey { vx: 0 };

        c.load(&[
            0x61, 0x02, // Load 2 into register 1
            0x71, 0x03, // Add 3 to register 1
        ]);

        assert_eq!(c.memory.v[1], 0);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START);

        c.advance_instruction()?;

        assert_eq!(c.memory.v[1], 0);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START);

        Ok(())
    }

    #[test]
    fn advance_instruction_waiting_dt() -> Result<()> {
        let mut c = Chip8::default();

        c.load(&[
            0x61, 0x02, // Load 2 into register 1
            0x71, 0x03, // Add 3 to register 1
        ]);
        c.memory.dt = 1;

        assert_eq!(c.memory.v[1], 0);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START);

        c.advance_instruction()?;

        assert_eq!(c.memory.v[1], 0);
        assert_eq!(c.memory.pc, Memory::INDEX_PROGRAM_START);

        Ok(())
    }

    // TODO(nenikitov): Test key pressing
}
