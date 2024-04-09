use super::*;
use crate::chip_8::*;
use thiserror::Error;

/// Errors encountered during execution of an instruction.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExecuteError {
    #[error("instruction {0:?} is not supported")]
    UnsupportedInstruction(Instruction),
    #[error("key {0:?} is not in 0-F range")]
    InvalidKey(u8),
}

pub trait ExecuteInstruction {
    /// Execute a given instruction.
    ///
    /// # Errors
    ///
    /// If the instruction did not execute correctly.
    fn execute(&mut self, instruction: &Instruction) -> Result<(), ExecuteError>;
}

impl ExecuteInstruction for Chip8 {
    fn execute(&mut self, instruction: &Instruction) -> Result<(), ExecuteError> {
        let memory = &mut self.memory;
        let config = &self.config;

        match *instruction {
            Instruction::DisplayClear => {
                memory.clear_vram();
            }
            Instruction::SubroutineReturn => {
                if let Some(pc) = memory.stack.pop() {
                    memory.pc = pc;
                } else {
                    todo!("Figure out what to do on the last return");
                }
            }
            Instruction::System { address: _ } => {
                return Err(ExecuteError::UnsupportedInstruction(*instruction))
            }
            Instruction::Jump { address } => {
                memory.pc = address;
            }
            Instruction::SubroutineCall { address } => {
                memory.stack.push(memory.pc);
                memory.pc = address;
            }
            Instruction::SkipIfVxEqualsValue { vx, value } => {
                if memory.v[vx] == value {
                    memory.increment_pc();
                }
            }
            Instruction::SkipIfVxNotEqualsValue { vx, value } => {
                if memory.v[vx] != value {
                    memory.increment_pc();
                }
            }
            Instruction::SkipIfVxEqualsVy { vx, vy } => {
                if memory.v[vx] == memory.v[vy] {
                    memory.increment_pc();
                }
            }
            Instruction::SetVxWithValue { vx, value } => {
                memory.v[vx] = value;
            }
            Instruction::AddVxValue { vx, value } => {
                memory.v[vx] = memory.v[vx].wrapping_add(value);
            }
            Instruction::SetVxWithVy { vx, vy } => {
                memory.v[vx] = memory.v[vy];
            }
            Instruction::OrVxWithVy { vx, vy } => {
                memory.v[vx] |= memory.v[vy];
            }
            Instruction::AndVxWithVy { vx, vy } => {
                memory.v[vx] &= memory.v[vy];
            }
            Instruction::XorVxWithVy { vx, vy } => {
                memory.v[vx] ^= memory.v[vy];
            }
            Instruction::AddVxWithVy { vx, vy } => {
                let (result, overflow) = memory.v[vx].overflowing_add(memory.v[vy]);
                memory.v[vx] = result;
                memory.v[Memory::INDEX_FLAG_REGISTER] = overflow.into();
            }
            Instruction::SubtractVxWithVy { vx, vy } => {
                let (result, underflow) = memory.v[vx].overflowing_sub(memory.v[vy]);
                memory.v[vx] = result;
                memory.v[Memory::INDEX_FLAG_REGISTER] = (!underflow).into();
            }
            Instruction::Shift1RightVxWithVy { vx, vy } => {
                if !config.shift_ignores_vy {
                    memory.v[vx] = memory.v[vy];
                }

                let discarded = memory.v[vx] & 0b00000001;

                memory.v[vx] >>= 1;
                memory.v[Memory::INDEX_FLAG_REGISTER] = discarded;
            }
            Instruction::SubtractVyWithVx { vx, vy } => {
                let (result, underflow) = memory.v[vy].overflowing_sub(memory.v[vx]);
                memory.v[vx] = result;
                memory.v[Memory::INDEX_FLAG_REGISTER] = (!underflow).into();
            }
            Instruction::Shift1LeftVxWithVy { vx, vy } => {
                if !config.shift_ignores_vy {
                    memory.v[vx] = memory.v[vy];
                }

                let discarded = (memory.v[vx] & 0b10000000) >> 7;

                memory.v[vx] <<= 1;
                memory.v[Memory::INDEX_FLAG_REGISTER] = discarded;
            }
            Instruction::SkipIfVxNotEqualsVy { vx, vy } => {
                if memory.v[vx] != memory.v[vy] {
                    memory.increment_pc();
                }
            }
            Instruction::SetIWithValue { value } => {
                memory.i = value;
            }
            Instruction::JumpWithOffset { vx, address: value } => {
                let register_offset = memory.v[if config.jump_reads_from_vx { vx } else { 0 }];
                memory.pc = value + register_offset as u16;
            }
            Instruction::SetVxWithRandom { vx, value } => {
                memory.v[vx] = rand::random::<u8>() & value;
            }
            Instruction::DisplayDraw { vx, vy, height } => {
                let x = memory.v[vx] % Memory::SIZE_DISPLAY_WIDTH as u8;
                let y = memory.v[vy] % Memory::SIZE_DISPLAY_HEIGHT as u8;
                memory.v[Memory::INDEX_FLAG_REGISTER] = 0;
                'rows: for r in 0..(height) {
                    let row = memory.ram[(memory.i + r as u16) as usize];
                    'pixels: for p in 0..8 {
                        let pixel = row & (1 << (7 - p));
                        let pixel = pixel != 0;
                        if pixel {
                            let x = (x + p) as usize;
                            let y = (y + r) as usize;
                            if x >= Memory::SIZE_DISPLAY_WIDTH {
                                break 'pixels;
                            }
                            if y >= Memory::SIZE_DISPLAY_HEIGHT {
                                break 'rows;
                            }
                            memory.vram[y][x] ^= pixel;
                            if !memory.vram[y][x] {
                                memory.v[Memory::INDEX_FLAG_REGISTER] = 1;
                            }
                        }
                    }
                }
            }
            Instruction::SkipIfVxKeyPressed { vx } => {
                if let Some(&key) = memory.keys.get(memory.v[vx] as usize) {
                    if key {
                        memory.increment_pc();
                    }
                } else {
                    return Err(ExecuteError::InvalidKey(memory.v[vx]));
                }
            }
            Instruction::SkipIfVxKeyNotPressed { vx } => {
                if let Some(&key) = memory.keys.get(memory.v[vx] as usize) {
                    if !key {
                        memory.increment_pc();
                    }
                } else {
                    return Err(ExecuteError::InvalidKey(memory.v[vx]));
                }
            }
            Instruction::SetVxWithDt { vx } => {
                memory.v[vx] = memory.dt;
            }
            Instruction::SetVxWithNextPressedKeyBlocking { vx } => {
                self.state = State::WaitingForKey { vx };
            }
            Instruction::SetDtWithVx { vx } => {
                memory.dt = memory.v[vx];
            }
            Instruction::SetStWithVx { vx } => {
                memory.st = memory.v[vx];
            }
            Instruction::AddIWithVx { vx } => {
                memory.i += memory.v[vx] as u16;

                if self.config.add_to_index_stores_overflow && memory.i >= 0x1000 {
                    memory.v[Memory::INDEX_FLAG_REGISTER] = 1;
                }
            }
            Instruction::SetIWithCharacterAtVx { vx } => {
                memory.i = Memory::INDEX_FONT_START as u16 + memory.v[vx] as u16 * 5;
            }
        };

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
        chip.memory.dt = 60;
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
    fn execute_display_clear(mut target: Chip8, mut result: Chip8) -> Result<()> {
        target.execute(&Instruction::DisplayClear)?;

        result.memory.vram = [[false; Memory::SIZE_DISPLAY_WIDTH]; Memory::SIZE_DISPLAY_HEIGHT];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subroutine_return_once(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] address_1: u16,
        #[values(0x123, 0x234)] address_2: u16,
    ) -> Result<()> {
        target.execute(&Instruction::SubroutineCall { address: address_1 })?;
        target.execute(&Instruction::SubroutineCall { address: address_2 })?;
        target.execute(&Instruction::SubroutineReturn)?;

        result.memory.stack.push(Memory::INDEX_PROGRAM_START);
        result.memory.pc = address_1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subroutine_return_twice(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] address_1: u16,
        #[values(0x123, 0x234)] address_2: u16,
    ) -> Result<()> {
        target.execute(&Instruction::SubroutineCall { address: address_1 })?;
        target.execute(&Instruction::SubroutineCall { address: address_2 })?;
        target.execute(&Instruction::SubroutineReturn)?;
        target.execute(&Instruction::SubroutineReturn)?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_system_unsupported(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] address: u16,
    ) -> Result<()> {
        let instruction = Instruction::System { address };

        assert_eq!(
            target.execute(&instruction),
            Err(ExecuteError::UnsupportedInstruction(instruction))
        );
        Ok(())
    }

    #[rstest]
    fn execute_jump(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] address: u16,
    ) -> Result<()> {
        target.execute(&Instruction::Jump { address })?;

        result.memory.pc = address;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subroutine_call_once(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] address: u16,
    ) -> Result<()> {
        target.execute(&Instruction::SubroutineCall { address })?;

        result.memory.stack.push(Memory::INDEX_PROGRAM_START);
        result.memory.pc = address;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subroutine_call_twice(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] address_1: u16,
        #[values(0x123, 0x234)] address_2: u16,
    ) -> Result<()> {
        target.execute(&Instruction::SubroutineCall { address: address_1 })?;
        target.execute(&Instruction::SubroutineCall { address: address_2 })?;

        result
            .memory
            .stack
            .append(&mut vec![Memory::INDEX_PROGRAM_START, address_1]);
        result.memory.pc = address_2;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_equals_value_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxEqualsValue {
            vx,
            value: target.memory.v[vx],
        })?;

        result.memory.increment_pc();

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_equals_value_not_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxEqualsValue {
            vx,
            value: target.memory.v[vx] + 1,
        })?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_not_equals_value_not_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxNotEqualsValue {
            vx,
            value: target.memory.v[vx] + 1,
        })?;

        result.memory.increment_pc();

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_not_equals_value_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxNotEqualsValue {
            vx,
            value: target.memory.v[vx],
        })?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_equals_vy_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxEqualsVy { vx, vy: vx + 8 })?;

        result.memory.increment_pc();

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_equals_vy_not_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxEqualsVy { vx, vy: vx + 9 })?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_vx_with_value(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        target.execute(&Instruction::SetVxWithValue { vx, value })?;

        result.memory.v[vx] = value;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_vx_value(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        target.execute(&Instruction::AddVxValue { vx, value })?;

        result.memory.v[vx] += value;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_vx_value_overflow(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::AddVxValue { vx, value: 255 })?;

        result.memory.v[vx] -= 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_vx_with_vy(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SetVxWithVy { vx, vy })?;

        result.memory.v[vx] = result.memory.v[vy];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_or_vx_with_vy(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::OrVxWithVy { vx, vy })?;

        result.memory.v[vx] |= result.memory.v[vy];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_and_vx_with_vy(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::AndVxWithVy { vx, vy })?;

        result.memory.v[vx] &= result.memory.v[vy];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_xor_vx_with_vy(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::XorVxWithVy { vx, vy })?;

        result.memory.v[vx] ^= result.memory.v[vy];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_vx_with_vy(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::AddVxWithVy { vx, vy })?;

        result.memory.v[vx] += result.memory.v[vy];
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 0;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_vx_with_vy_overflow(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.memory.v[vy] = 255;
        target.execute(&Instruction::AddVxWithVy { vx, vy })?;

        result.memory.v[vx] -= 1;
        result.memory.v[vy] = 255;
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subtract_vx_with_vy(
        mut target: Chip8,
        mut result: Chip8,
        #[values(6, 7)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SubtractVxWithVy { vx, vy })?;

        result.memory.v[vx] -= result.memory.v[vy];
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subtract_vx_with_vy_overflow(
        mut target: Chip8,
        mut result: Chip8,
        #[values(3, 4)] vx: usize,
        #[values(6, 7)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SubtractVxWithVy { vx, vy })?;

        result.memory.v[vx] = result.memory.v[vx].wrapping_sub(result.memory.v[vy]);
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 0;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_shift_1_right_vx_with_vy_compat_ignore_vy(
        #[with(Config { shift_ignores_vy: true, ..Config::default() })] mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::Shift1RightVxWithVy { vx, vy })?;

        result.memory.v[Memory::INDEX_FLAG_REGISTER] = result.memory.v[vx] & 0b00000001;
        result.memory.v[vx] >>= 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_shift_1_right_vx_with_vy_compat_use_vy(
        #[with(Config { shift_ignores_vy: false, ..Config::default() })] mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::Shift1RightVxWithVy { vx, vy })?;

        result.memory.v[Memory::INDEX_FLAG_REGISTER] = result.memory.v[vy] & 0b00000001;
        result.memory.v[vx] = result.memory.v[vy] >> 1;

        assert_eq!(target, result);

        Ok(())
    }

    #[rstest]
    fn execute_subtract_vy_with_vx(
        mut target: Chip8,
        mut result: Chip8,
        #[values(3, 4)] vx: usize,
        #[values(6, 7)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SubtractVyWithVx { vx, vy })?;

        result.memory.v[vx] = result.memory.v[vy] - result.memory.v[vx];
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_subtract_vy_with_vx_overflow(
        mut target: Chip8,
        mut result: Chip8,
        #[values(6, 7)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SubtractVyWithVx { vx, vy })?;

        result.memory.v[vx] = result.memory.v[vy].wrapping_sub(result.memory.v[vx]);
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 0;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_shift_1_left_vx_with_vy_compat_ignore_vy(
        #[with(Config { shift_ignores_vy: true, ..Config::default() })] mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::Shift1LeftVxWithVy { vx, vy })?;

        result.memory.v[Memory::INDEX_FLAG_REGISTER] = (result.memory.v[vx] & 0b10000000) >> 7;
        result.memory.v[vx] <<= 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_shift_1_left_vx_with_vy_compat_use_vy(
        #[with(Config { shift_ignores_vy: false, ..Config::default() })] mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(3, 4)] vy: usize,
    ) -> Result<()> {
        target.execute(&Instruction::Shift1LeftVxWithVy { vx, vy })?;

        result.memory.v[Memory::INDEX_FLAG_REGISTER] = (result.memory.v[vy] & 0b10000000) >> 7;
        result.memory.v[vx] = result.memory.v[vy] << 1;

        assert_eq!(target, result);

        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_not_equals_vy_not_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxNotEqualsVy { vx, vy: vx + 9 })?;

        result.memory.increment_pc();

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_not_vy_value_equals(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxNotEqualsVy { vx, vy: vx + 8 })?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_i_with_value(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0x123, 0x234)] value: u16,
    ) -> Result<()> {
        target.execute(&Instruction::SetIWithValue { value })?;

        result.memory.i = value;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_jump_with_offset_compat_use_v0(
        #[with(Config { jump_reads_from_vx: false, ..Config::default() })] mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(0x123, 0x234)] address: u16,
    ) -> Result<()> {
        target.execute(&Instruction::JumpWithOffset { vx, address })?;

        result.memory.pc = address + result.memory.v[0] as u16;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_jump_with_offset_compat_use_vx(
        #[with(Config { jump_reads_from_vx: true, ..Config::default() })] mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(0x123, 0x234)] address: u16,
    ) -> Result<()> {
        target.execute(&Instruction::JumpWithOffset { vx, address })?;

        result.memory.pc = address + result.memory.v[vx] as u16;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_vx_with_random(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 2)] vx: usize,
        #[values(0b11001100, 0b10101010)] value: u8,
    ) -> Result<()> {
        target.execute(&Instruction::SetVxWithRandom { vx, value })?;

        result.memory.v[vx] = target.memory.v[vx];

        assert_eq!(target, result);
        assert_eq!(target.memory.v[vx] & (!value), 0);
        Ok(())
    }

    #[rstest]
    fn execute_display_draw(
        mut target: Chip8,
        mut result: Chip8,
        #[values(2, 3, 7)] vx: usize,
        #[values(3, 3, 6)] vy: usize,
    ) -> Result<()> {
        let x = target.memory.v[vx] as usize;
        let y = target.memory.v[vy] as usize;
        let width = usize::min(8, Memory::SIZE_DISPLAY_WIDTH - x);

        target.memory.ram[target.memory.i as usize + 0] = 0b10111111;
        target.memory.ram[target.memory.i as usize + 1] = 0b01001001;

        target.execute(&Instruction::DisplayDraw { vx, vy, height: 2 })?;

        result.memory.ram[result.memory.i as usize + 0] = 0b10111111;
        result.memory.ram[result.memory.i as usize + 1] = 0b01001001;
        result.memory.vram[y][x..][..width]
            .copy_from_slice(&[true, false, true, true, true, true, true, true][..width]);
        if y + 1 < Memory::SIZE_DISPLAY_HEIGHT {
            result.memory.vram[y + 1][x..][..width]
                .copy_from_slice(&[false, true, false, false, true, false, false, true][..width]);
        }
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 0;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_display_draw_overdraw(
        mut target: Chip8,
        mut result: Chip8,
        #[values(2, 3, 7)] vx: usize,
        #[values(3, 3, 6)] vy: usize,
    ) -> Result<()> {
        let x = target.memory.v[vx] as usize;
        let y = target.memory.v[vy] as usize;
        let width = usize::min(8, Memory::SIZE_DISPLAY_WIDTH - x);

        target.memory.ram[target.memory.i as usize + 0] = 0b10111111;
        target.memory.ram[target.memory.i as usize + 1] = 0b01001001;
        target.memory.vram[y][x] = true;

        target.execute(&Instruction::DisplayDraw { vx, vy, height: 2 })?;

        result.memory.ram[result.memory.i as usize + 0] = 0b10111111;
        result.memory.ram[result.memory.i as usize + 1] = 0b01001001;
        result.memory.vram[y][x..][..width]
            .copy_from_slice(&[false, false, true, true, true, true, true, true][..width]);
        if y + 1 < Memory::SIZE_DISPLAY_HEIGHT {
            result.memory.vram[y + 1][x..][..width]
                .copy_from_slice(&[false, true, false, false, true, false, false, true][..width]);
        }
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_key_pressed_pressed(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxKeyPressed { vx })?;

        result.memory.increment_pc();

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_key_pressed_not_pressed(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 3)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxKeyPressed { vx })?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_key_not_pressed_not_pressed(
        mut target: Chip8,
        mut result: Chip8,
        #[values(1, 3)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxKeyNotPressed { vx })?;

        result.memory.increment_pc();

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_skip_if_vx_key_not_pressed_pressed(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SkipIfVxKeyNotPressed { vx })?;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_vx_with_dt(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SetVxWithDt { vx })?;

        result.memory.v[vx] = result.memory.dt;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_vx_with_next_pressed_key_blocking(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 1)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SetVxWithNextPressedKeyBlocking { vx })?;

        result.state = State::WaitingForKey { vx };

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_dt_with_vx(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SetDtWithVx { vx })?;

        result.memory.dt = result.memory.v[vx];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_st_with_vx(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SetStWithVx { vx })?;

        result.memory.st = result.memory.v[vx];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_i_with_vx_compat_store_overflow(
        #[with(Config { add_to_index_stores_overflow: true, ..Config::default() })]
        mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::AddIWithVx { vx })?;

        result.memory.i += result.memory.v[vx] as u16;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_i_with_vx_overflow_compat_store_overflow(
        #[with(Config { add_to_index_stores_overflow: true, ..Config::default() })]
        mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.memory.i = 0x1000;

        target.execute(&Instruction::AddIWithVx { vx })?;

        result.memory.i = 0x1000 + result.memory.v[vx] as u16;
        result.memory.v[Memory::INDEX_FLAG_REGISTER] = 1;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_i_with_vx_compat_ignore_overflow(
        #[with(Config { add_to_index_stores_overflow: false, ..Config::default() })]
        mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::AddIWithVx { vx })?;

        result.memory.i += result.memory.v[vx] as u16;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_add_i_with_vx_overflow_compat_ignore_overflow(
        #[with(Config { add_to_index_stores_overflow: false, ..Config::default() })]
        mut target: Chip8,
        #[with(target.clone())] mut result: Chip8,
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        target.memory.i = 0x1000;

        target.execute(&Instruction::AddIWithVx { vx })?;

        result.memory.i = 0x1000 + result.memory.v[vx] as u16;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn execute_set_i_with_character_at_vx(
        mut target: Chip8,
        mut result: Chip8,
        #[values(0, 2)] vx: usize,
    ) -> Result<()> {
        target.execute(&Instruction::SetIWithCharacterAtVx { vx })?;

        result.memory.i = Memory::INDEX_FONT_START as u16 + result.memory.v[vx] as u16 * 5;

        assert_eq!(target, result);
        Ok(())
    }
}
