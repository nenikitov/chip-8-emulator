use super::*;
use crate::chip_8::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExecuteError {
    #[error("instruction {0:?} is not supported")]
    UnsupportedInstruction(Instruction),
}

/// Instruction that can be executes on memory.
pub trait ExecuteOnChip8 {
    fn execute(&self, chip: &mut Chip8) -> Result<(), ExecuteError>;
}

impl ExecuteOnChip8 for Instruction {
    fn execute(&self, chip: &mut Chip8) -> Result<(), ExecuteError> {
        match *self {
            Instruction::System { address: _ } => {
                return Err(ExecuteError::UnsupportedInstruction(*self))
            }
            Instruction::DisplayClear => chip
                .vram
                .iter_mut()
                .for_each(|e| e.iter_mut().for_each(|e| *e = false)),
            Instruction::Jump { address } => {
                chip.pc = address;
            }
            Instruction::SetVxWithValue { vx, value } => {
                chip.v[vx] = value;
            }
            Instruction::AddVxValue { vx, value } => {
                chip.v[vx] = chip.v[vx].wrapping_add(value);
            }
            Instruction::SetIWithValue { value } => {
                chip.i = value;
            }
            Instruction::DisplayDraw { vx, vy, height } => {
                let x = chip.v[vx] % SIZE_DISPLAY.0 as u8;
                let y = chip.v[vy] % SIZE_DISPLAY.1 as u8;
                chip.v[0xF] = 0;
                'rows: for r in 0..(height) {
                    let row = chip.ram[(chip.i + r as u16) as usize];
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
                            chip.vram[y][x] ^= pixel;
                            if !chip.vram[y][x] {
                                chip.v[0xF] = 1;
                            }
                        }
                    }
                }
            }
            Instruction::SubroutineReturn => {
                if let Some(pc) = chip.stack.pop() {
                    chip.pc = pc
                } else {
                    todo!("Figure out what to do on the last return");
                }
            }
            Instruction::SubroutineCall { address } => {
                chip.stack.push(chip.pc);
                chip.pc = address
            }
            Instruction::SkipIfVxEquals { vx, value } => {
                if chip.v[vx] == value {
                    chip.increment_pc();
                }
            }
            Instruction::SkipIfVxNotEquals { vx, value } => {
                if chip.v[vx] != value {
                    chip.increment_pc();
                }
            }
            Instruction::SkipIfVxEqualsVy { vx, vy } => {
                if chip.v[vx] == chip.v[vy] {
                    chip.increment_pc();
                }
            }
            Instruction::SkipIfVxNotEqualsVy { vx, vy } => {
                if chip.v[vx] != chip.v[vy] {
                    chip.increment_pc();
                }
            }
            Instruction::SetVxWithVy { vx, vy } => {
                chip.v[vx] = chip.v[vy];
            }
            Instruction::OrVxWithVy { vx, vy } => {
                chip.v[vx] |= chip.v[vy];
            }
            Instruction::AndVxWithVy { vx, vy } => {
                chip.v[vx] &= chip.v[vy];
            }
            Instruction::XorVxWithVy { vx, vy } => {
                chip.v[vx] ^= chip.v[vy];
            }
            Instruction::AddVxWithVy { vx, vy } => {
                let (result, overflow) = chip.v[vx].overflowing_add(chip.v[vy]);
                chip.v[vx] = result;
                chip.v[0xF] = overflow.into();
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
            Instruction::System { address: 0x123 }.execute(&mut c),
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

        Instruction::DisplayClear.execute(&mut c)?;

        assert_eq!(c.vram, [[false; SIZE_DISPLAY.0]; SIZE_DISPLAY.1]);

        Ok(())
    }

    #[test]
    fn execute_jump() -> Result<()> {
        let mut c = Chip8::default();

        Instruction::Jump { address: 0x123 }.execute(&mut c)?;

        assert_eq!(c.pc, 0x123);

        Ok(())
    }

    #[test]
    fn execute_set_vx_with_value() -> Result<()> {
        let mut c = Chip8::default();

        Instruction::SetVxWithValue { vx: 5, value: 0x32 }.execute(&mut c)?;

        assert_eq!(c.v[5], 0x32);

        Ok(())
    }

    #[test]
    fn execute_add_vx_value() -> Result<()> {
        let mut c = Chip8::default();
        c.v[4] = 1;

        Instruction::AddVxValue { vx: 4, value: 0x33 }.execute(&mut c)?;

        assert_eq!(c.v[4], 0x34);

        Ok(())
    }

    #[test]
    fn execute_add_vx_value_overflow() -> Result<()> {
        let mut c = Chip8::default();
        c.v[4] = 0xFF;
        c.v[0xF] = 0x30;

        Instruction::AddVxValue { vx: 4, value: 0x2 }.execute(&mut c)?;

        assert_eq!(c.v[4], 0x1);
        assert_eq!(c.v[0xF], 0x30);

        Ok(())
    }

    #[test]
    fn execute_set_i_with_value() -> Result<()> {
        let mut c = Chip8::default();

        Instruction::SetIWithValue { value: 0x123 }.execute(&mut c)?;

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

        Instruction::DisplayDraw {
            vx: 4,
            vy: 6,
            height: 2,
        }
        .execute(&mut c)?;

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

        Instruction::Jump { address: 0x123 }.execute(&mut c)?;

        Instruction::SubroutineCall { address: 0x234 }.execute(&mut c)?;
        assert_eq!(c.pc, 0x234);
        assert_eq!(c.stack, vec![0x123]);

        Instruction::SubroutineCall { address: 0x345 }.execute(&mut c)?;
        assert_eq!(c.pc, 0x345);
        assert_eq!(c.stack, vec![0x123, 0x234]);

        Ok(())
    }

    #[test]
    fn execute_subroutine_return() -> Result<()> {
        let mut c = Chip8::default();

        Instruction::Jump { address: 0x123 }.execute(&mut c)?;
        Instruction::SubroutineCall { address: 0x234 }.execute(&mut c)?;
        Instruction::SubroutineCall { address: 0x345 }.execute(&mut c)?;

        Instruction::SubroutineReturn.execute(&mut c)?;
        assert_eq!(c.pc, 0x234);
        assert_eq!(c.stack, vec![0x123]);

        Instruction::SubroutineReturn.execute(&mut c)?;
        assert_eq!(c.pc, 0x123);
        assert_eq!(c.stack, vec![]);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_equals() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;

        Instruction::SkipIfVxEquals {
            vx: 0x2,
            value: 0x0,
        }
        .execute(&mut c)?;
        assert_eq!(c.pc, 14);

        Instruction::SkipIfVxEquals {
            vx: 0x2,
            value: 0x34,
        }
        .execute(&mut c)?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_not_equals() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;

        Instruction::SkipIfVxNotEquals {
            vx: 0x2,
            value: 0x34,
        }
        .execute(&mut c)?;
        assert_eq!(c.pc, 14);

        Instruction::SkipIfVxNotEquals {
            vx: 0x2,
            value: 0x0,
        }
        .execute(&mut c)?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_equals_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;
        c.v[0x3] = 0x17;

        Instruction::SkipIfVxEqualsVy { vx: 0x2, vy: 0x3 }.execute(&mut c)?;
        assert_eq!(c.pc, 14);

        c.v[0x3] = 0x34;
        Instruction::SkipIfVxEqualsVy { vx: 0x2, vy: 0x3 }.execute(&mut c)?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_skip_if_vx_not_equals_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.pc = 14;
        c.v[0x2] = 0x34;
        c.v[0x3] = 0x34;

        Instruction::SkipIfVxNotEqualsVy { vx: 0x2, vy: 0x3 }.execute(&mut c)?;
        assert_eq!(c.pc, 14);

        c.v[0x3] = 0x17;
        Instruction::SkipIfVxNotEqualsVy { vx: 0x2, vy: 0x3 }.execute(&mut c)?;
        assert_eq!(c.pc, 16);

        Ok(())
    }

    #[test]
    fn execute_set_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0x10;
        c.v[2] = 0x20;

        Instruction::SetVxWithVy { vx: 1, vy: 2 }.execute(&mut c)?;

        assert_eq!(c.v[1], 0x20);
        assert_eq!(c.v[2], 0x20);

        Ok(())
    }

    #[test]
    fn execute_or_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0b101100;
        c.v[2] = 0b010110;

        Instruction::OrVxWithVy { vx: 1, vy: 2 }.execute(&mut c)?;

        assert_eq!(c.v[1], 0b111110);
        assert_eq!(c.v[2], 0b010110);

        Ok(())
    }

    #[test]
    fn execute_and_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0b101100;
        c.v[2] = 0b010110;

        Instruction::AndVxWithVy { vx: 1, vy: 2 }.execute(&mut c)?;

        assert_eq!(c.v[1], 0b000100);
        assert_eq!(c.v[2], 0b010110);

        Ok(())
    }

    #[test]
    fn execute_xor_vx_with_vy() -> Result<()> {
        let mut c = Chip8::default();

        c.v[1] = 0b101100;
        c.v[2] = 0b010110;

        Instruction::XorVxWithVy { vx: 1, vy: 2 }.execute(&mut c)?;

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

        Instruction::AddVxWithVy { vx: 1, vy: 2 }.execute(&mut c)?;

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

        Instruction::AddVxWithVy { vx: 1, vy: 2 }.execute(&mut c)?;

        assert_eq!(c.v[1], 1);
        assert_eq!(c.v[2], 2);
        assert_eq!(c.v[0xF], 1);

        Ok(())
    }
}
