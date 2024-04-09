use thiserror::Error;

use crate::instruction::*;

use super::*;

/// Combines [`ParseError`] and [`ExecuteError`]
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

/// Main structure used to emulate CHIP-8.
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
            self.state = State::Ready;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use eyre::Result;
    use rstest::*;
    use similar_asserts::assert_eq;

    #[fixture]
    fn target(#[default(Config::default())] config: Config) -> Chip8 {
        let mut chip = Chip8::new(config);

        chip.memory.ram[Memory::INDEX_PROGRAM_START as usize..][..4].copy_from_slice(&[
            0x61, 0x02, // Load 2 into register 1
            0x71, 0x03, // Add 3 to it
        ]);
        chip.memory.vram[0].iter_mut().for_each(|e| *e = true);
        chip.memory.stack.push(Memory::INDEX_PROGRAM_START);
        chip.memory.dt = 0;
        chip.memory.st = 10;
        chip.memory.i = 100;
        chip.memory.v = [0, 1, 2, 3, 4, 5, 31, 59, 0, 1, 2, 3, 4, 5, 30, 60];
        chip.memory.keys = [
            true, false, true, false, true, false, true,
            false, // First 8 are pressed and unpressed
            false, false, false, false, false, false, false, false, // Last 8 are not pressed
        ];

        chip
    }

    #[fixture]
    fn result(target: Chip8) -> Chip8 {
        target.clone()
    }

    #[rstest]
    fn advance_instruction_ready(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.advance_instruction()?;
        target.advance_instruction()?;

        result.memory.v[1] = 5;
        result.memory.pc += 4;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn advance_instruction_waiting_key(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.state = State::WaitingForKey { vx: 0x0 };
        target.advance_instruction()?;
        target.advance_instruction()?;

        result.state = State::WaitingForKey { vx: 0x0 };

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn advance_instruction_waiting_dt(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.memory.dt = 10;
        target.advance_instruction()?;
        target.advance_instruction()?;

        result.memory.dt = 10;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn advance_timer(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.memory.dt = 10;
        for _ in 0..3 {
            target.advance_timer();
        }

        result.memory.dt = 7;
        result.memory.st -= 3;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn advance_timer_waiting_key(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.memory.dt = 10;
        target.state = State::WaitingForKey { vx: 0x0 };
        for _ in 0..3 {
            target.advance_timer();
        }

        result.state = State::WaitingForKey { vx: 0x0 };
        result.memory.dt = 7;
        result.memory.st -= 3;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn press_key(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.press_key(0xF);

        result.memory.keys[0xF] = true;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn unpress_key(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.unpress_key(0x0);

        result.memory.keys[0x0] = false;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn unpress_key_unblocks_machine_and_stores_pressed_key(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(0x0, 0x2)] key: u8,
    ) -> Result<()> {
        target.state = State::WaitingForKey { vx };
        target.unpress_key(key);

        result.memory.keys[key as usize] = false;
        result.memory.v[vx] = key;

        assert_eq!(target, result);
        Ok(())
    }
}
