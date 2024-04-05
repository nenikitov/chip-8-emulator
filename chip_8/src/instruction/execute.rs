use super::*;
use crate::chip_8::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExecuteError {
    #[error("instruction {0:?} is not supported")]
    UnsupportedInstruction(Instruction),
}

pub trait ExecuteInstruction {
    fn execute(&mut self, instruction: &Instruction) -> Result<(), ExecuteError>;
}

impl ExecuteInstruction for Chip8 {
    fn execute(&mut self, instruction: &Instruction) -> Result<(), ExecuteError> {
        match *instruction {
            Instruction::System { address: _ } => {
                return Err(ExecuteError::UnsupportedInstruction(*instruction))
            }
            Instruction::DisplayClear => self
                .vram
                .iter_mut()
                .for_each(|e| e.iter_mut().for_each(|e| *e = false)),
            Instruction::Jump { address } => {
                self.pc = address;
            }
            Instruction::SetVxWithValue { vx, value } => {
                self.v[vx] = value;
            }
            Instruction::AddVxValue { vx, value } => {
                self.v[vx] = self.v[vx].wrapping_add(value);
            }
            Instruction::SetIWithValue { value } => {
                self.i = value;
            }
            Instruction::DisplayDraw { vx, vy, height } => {
                let x = self.v[vx] % SIZE_DISPLAY.0 as u8;
                let y = self.v[vy] % SIZE_DISPLAY.1 as u8;
                self.v[0xF] = 0;
                'rows: for r in 0..(height) {
                    let row = self.ram[(self.i + r as u16) as usize];
                    'pixels: for p in 0..8 {
                        let pixel = row & (1 << (7 - p));
                        let pixel = pixel != 0;
                        if pixel {
                            let x = (x + p) as usize;
                            let y = (y + r) as usize;
                            if x >= SIZE_DISPLAY.0 {
                                break 'pixels;
                            }
                            if y >= SIZE_DISPLAY.1 {
                                break 'rows;
                            }
                            self.vram[y][x] ^= pixel;
                            if !self.vram[y][x] {
                                self.v[0xF] = 1;
                            }
                        }
                    }
                }
            }
            Instruction::SubroutineReturn => {
                if let Some(pc) = self.stack.pop() {
                    self.pc = pc
                } else {
                    todo!("Figure out what to do on the last return");
                }
            }
            Instruction::SubroutineCall { address } => {
                self.stack.push(self.pc);
                self.pc = address
            }
            Instruction::SkipIfVxEquals { vx, value } => {
                if self.v[vx] == value {
                    self.increment_pc();
                }
            }
            Instruction::SkipIfVxNotEquals { vx, value } => {
                if self.v[vx] != value {
                    self.increment_pc();
                }
            }
            Instruction::SkipIfVxEqualsVy { vx, vy } => {
                if self.v[vx] == self.v[vy] {
                    self.increment_pc();
                }
            }
            Instruction::SkipIfVxNotEqualsVy { vx, vy } => {
                if self.v[vx] != self.v[vy] {
                    self.increment_pc();
                }
            }
            Instruction::SetVxWithVy { vx, vy } => {
                self.v[vx] = self.v[vy];
            }
            Instruction::OrVxWithVy { vx, vy } => {
                self.v[vx] |= self.v[vy];
            }
            Instruction::AndVxWithVy { vx, vy } => {
                self.v[vx] &= self.v[vy];
            }
            Instruction::XorVxWithVy { vx, vy } => {
                self.v[vx] ^= self.v[vy];
            }
            Instruction::AddVxWithVy { vx, vy } => {
                let (result, overflow) = self.v[vx].overflowing_add(self.v[vy]);
                self.v[vx] = result;
                self.v[0xF] = overflow.into();
            }
            Instruction::SubtractVxWithVy { vx, vy } => {
                let (result, underflow) = self.v[vx].overflowing_sub(self.v[vy]);
                self.v[vx] = result;
                self.v[0xF] = (!underflow).into();
            }
            Instruction::SubtractVyWithVx { vx, vy } => {
                let (result, underflow) = self.v[vy].overflowing_sub(self.v[vx]);
                self.v[vx] = result;
                self.v[0xF] = (!underflow).into();
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use eyre::Result;

    use super::*;

    #[test]
    fn execute_system_unsupported() -> Result<()> {
        let mut c = Chip8::default();
        assert_eq!(
            c.execute(&Instruction::System { address: 0x123 }),
            Err(ExecuteError::UnsupportedInstruction(Instruction::System {
                address: 0x123
            }))
        );

        Ok(())
    }

    #[test]
    fn execute_display_clear() -> Result<()> {
        let mut c = Chip8::default();
        c.vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = true));

        c.execute(&Instruction::DisplayClear)?;

        assert_eq!(c.vram, [[false; SIZE_DISPLAY.0]; SIZE_DISPLAY.1]);

        Ok(())
    }

    #[test]
    fn execute_jump() -> Result<()> {
        let mut c = Chip8::default();

        c.execute(&Instruction::Jump { address: 0x123 })?;

        assert_eq!(c.pc, 0x123);

        Ok(())
    }

    #[test]
    fn execute_set_vx_with_value() -> Result<()> {
        let mut c = Chip8::default();

        c.execute(&Instruction::SetVxWithValue { vx: 5, value: 0x32 })?;

        assert_eq!(c.v[5], 0x32);

        Ok(())
    }

    #[test]
    fn execute_add_vx_value() -> Result<()> {
        let mut c = Chip8::default();
        c.v[4] = 1;

        c.execute(&Instruction::AddVxValue { vx: 4, value: 0x33 })?;

        assert_eq!(c.v[4], 0x34);

        Ok(())
    }

    #[test]
    fn execute_add_vx_value_overflow() -> Result<()> {
        let mut c = Chip8::default();
        c.v[4] = 0xFF;
        c.v[0xF] = 0x30;

        c.execute(&Instruction::AddVxValue { vx: 4, value: 0x2 })?;

        assert_eq!(c.v[4], 0x1);
        assert_eq!(c.v[0xF], 0x30);

        Ok(())
    }

    #[test]
    fn execute_set_i_with_value() -> Result<()> {
        let mut c = Chip8::default();

        c.execute(&Instruction::SetIWithValue { value: 0x123 })?;

        assert_eq!(c.i, 0x123);

        Ok(())
    }

    #[test]
    fn execute_display_draw() -> Result<()> {
        let mut c = Chip8::default();
        c.i = 0;
        c.ram[0] = 0b10111111;
        c.ram[1] = 0b01001001;
        c.v[4] = 1;
        c.v[6] = 2;
        c.vram[2][1] = true;
        c.vram[2][2] = true;
        c.vram[2][3] = true;
        c.vram[3][1] = true;
        c.vram[3][2] = true;
        c.vram[3][3] = true;

        c.execute(&Instruction::DisplayDraw {
            vx: 4,
            vy: 6,
            height: 2,
        })?;

        assert_eq!(c.vram[2][1], false);
        assert_eq!(c.vram[2][2], true);
        assert_eq!(c.vram[2][3], false);
        assert_eq!(c.vram[2][4], true);
        assert_eq!(c.vram[2][5], true);
        assert_eq!(c.vram[2][6], true);
        assert_eq!(c.vram[2][7], true);
        assert_eq!(c.vram[2][8], true);
        assert_eq!(c.vram[3][1], true);
        assert_eq!(c.vram[3][2], false);
        assert_eq!(c.vram[3][3], true);
        assert_eq!(c.vram[3][4], false);
        assert_eq!(c.vram[3][5], true);
        assert_eq!(c.vram[3][6], false);
        assert_eq!(c.vram[3][7], false);
        assert_eq!(c.vram[3][8], true);
        assert_eq!(c.v[0xF], 1);

        Ok(())
    }

    #[test]
    fn execute_subroutine_call() -> Result<()> {
        let mut c = Chip8::default();

        c.execute(&Instruction::Jump { address: 0x123 })?;

        c.execute(&Instruction::SubroutineCall { address: 0x234 })?;
        assert_eq!(c.pc, 0x234);
        assert_eq!(c.stack, vec![0x123]);

        c.execute(&Instruction::SubroutineCall { address: 0x345 })?;
        assert_eq!(c.pc, 0x345);
        assert_eq!(c.stack, vec![0x123, 0x234]);

        Ok(())
    }

    #[test]
    fn execute_subroutine_return() -> Result<()> {
        let mut c = Chip8::default();

        c.execute(&Instruction::Jump { address: 0x123 })?;
        c.execute(&Instruction::SubroutineCall { address: 0x234 })?;
        c.execute(&Instruction::SubroutineCall { address: 0x345 })?;

        c.execute(&Instruction::SubroutineReturn)?;
        assert_eq!(c.pc, 0x234);
        assert_eq!(c.stack, vec![0x123]);

        c.execute(&Instruction::SubroutineReturn)?;
        assert_eq!(c.pc, 0x123);
        assert_eq!(c.stack, vec![]);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_equals() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;

        c.execute(&Instruction::SkipIfVxEquals {
            vx: 0x2,
            value: 0x0,
        })?;
        assert_eq!(c.pc, 14);

        c.execute(&Instruction::SkipIfVxEquals {
            vx: 0x2,
            value: 0x34,
        })?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_not_equals() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;

        c.execute(&Instruction::SkipIfVxNotEquals {
            vx: 0x2,
            value: 0x34,
        })?;
        assert_eq!(c.pc, 14);

        c.execute(&Instruction::SkipIfVxNotEquals {
            vx: 0x2,
            value: 0x0,
        })?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_equals_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;
        c.v[0x3] = 0x17;

        c.execute(&Instruction::SkipIfVxEqualsVy { vx: 0x2, vy: 0x3 })?;
        assert_eq!(c.pc, 14);

        c.v[0x3] = 0x34;
        c.execute(&Instruction::SkipIfVxEqualsVy { vx: 0x2, vy: 0x3 })?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_not_equals_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;
        c.v[0x3] = 0x34;

        c.execute(&Instruction::SkipIfVxNotEqualsVy { vx: 0x2, vy: 0x3 })?;
        assert_eq!(c.pc, 14);

        c.v[0x3] = 0x17;
        c.execute(&Instruction::SkipIfVxNotEqualsVy { vx: 0x2, vy: 0x3 })?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_set_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0x10;
        c.v[2] = 0x20;

        c.execute(&Instruction::SetVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 0x20);
        assert_eq!(c.v[2], 0x20);

        Ok(())
    }

    #[test]
    fn execute_or_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0b101100;
        c.v[2] = 0b010110;

        c.execute(&Instruction::OrVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 0b111110);
        assert_eq!(c.v[2], 0b010110);

        Ok(())
    }

    #[test]
    fn execute_and_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0b101100;
        c.v[2] = 0b010110;

        c.execute(&Instruction::AndVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 0b000100);
        assert_eq!(c.v[2], 0b010110);

        Ok(())
    }

    #[test]
    fn execute_xor_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0b101100;
        c.v[2] = 0b010110;

        c.execute(&Instruction::XorVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 0b111010);
        assert_eq!(c.v[2], 0b010110);

        Ok(())
    }

    #[test]
    fn execute_add_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 15;
        c.v[2] = 17;
        c.v[0xF] = 3;

        c.execute(&Instruction::AddVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 32);
        assert_eq!(c.v[2], 17);
        assert_eq!(c.v[0xF], 0);

        Ok(())
    }

    #[test]
    fn execute_add_vx_with_vy_overflow() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0xFF;
        c.v[2] = 2;
        c.v[0xF] = 3;

        c.execute(&Instruction::AddVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 1);
        assert_eq!(c.v[2], 2);
        assert_eq!(c.v[0xF], 1);

        Ok(())
    }

    #[test]
    fn execute_subtract_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 17;
        c.v[2] = 15;
        c.v[0xF] = 3;

        c.execute(&Instruction::SubtractVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 2);
        assert_eq!(c.v[2], 15);
        assert_eq!(c.v[0xF], 1);

        Ok(())
    }

    #[test]
    fn execute_subtract_vx_with_vy_overflow() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 15;
        c.v[2] = 17;
        c.v[0xF] = 3;

        c.execute(&Instruction::SubtractVxWithVy { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 254);
        assert_eq!(c.v[2], 17);
        assert_eq!(c.v[0xF], 0);

        Ok(())
    }

    #[test]
    fn execute_subtract_vy_with_vx() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 15;
        c.v[2] = 17;
        c.v[0xF] = 3;

        c.execute(&Instruction::SubtractVyWithVx { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 2);
        assert_eq!(c.v[2], 17);
        assert_eq!(c.v[0xF], 1);

        Ok(())
    }

    #[test]
    fn execute_subtract_vy_with_vx_overflow() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 17;
        c.v[2] = 15;
        c.v[0xF] = 3;

        c.execute(&Instruction::SubtractVyWithVx { vx: 1, vy: 2 })?;

        assert_eq!(c.v[1], 254);
        assert_eq!(c.v[2], 15);
        assert_eq!(c.v[0xF], 0);

        Ok(())
    }
}
