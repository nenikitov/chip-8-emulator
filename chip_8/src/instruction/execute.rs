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
    fn execute(&self, memory: &mut Chip8) -> Result<(), ExecuteError>;
}

impl ExecuteOnChip8 for Instruction {
    fn execute(&self, memory: &mut Chip8) -> Result<(), ExecuteError> {
        match *self {
            Instruction::System { address: _ } => {
                return Err(ExecuteError::UnsupportedInstruction(*self))
            }
            Instruction::DisplayClear => memory
                .vram
                .iter_mut()
                .for_each(|e| e.iter_mut().for_each(|e| *e = false)),
            Instruction::Jump { address } => {
                memory.pc = address;
            }
            Instruction::LoadVxValue { vx, value } => {
                memory.v[vx] = value;
            }
            Instruction::AddVxValue { vx, value } => {
                memory.v[vx] = memory.v[vx].wrapping_add(value);
            }
            Instruction::LoadIValue { value } => {
                memory.i = value;
            }
            Instruction::DisplayDraw { vx, vy, height } => {
                let x = memory.v[vx] % SIZE_DISPLAY.0 as u16;
                let y = memory.v[vy] % SIZE_DISPLAY.1 as u16;
                memory.v[0xF] = 0;
                'rows: for r in 0..(height) {
                    let row = memory.ram[(memory.i + r) as usize];
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
                            memory.vram[y][x] ^= pixel;
                            if !memory.vram[y][x] {
                                memory.v[0xF] = 1;
                            }
                        }
                    }
                }
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
    fn instruction_execute_system_unsupported() -> Result<()> {
        let mut m = Chip8::default();
        assert_eq!(
            Instruction::System { address: 0x123 }.execute(&mut m),
            Err(ExecuteError::UnsupportedInstruction(Instruction::System {
                address: 0x123
            }))
        );

        Ok(())
    }

    #[test]
    fn instruction_execute_display_clear() -> Result<()> {
        let mut m = Chip8::default();
        m.vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = true));

        Instruction::DisplayClear.execute(&mut m)?;

        assert_eq!(m.vram, [[false; SIZE_DISPLAY.0]; SIZE_DISPLAY.1]);

        Ok(())
    }

    #[test]
    fn instruction_execute_jump() -> Result<()> {
        let mut m = Chip8::default();

        Instruction::Jump { address: 0x123 }.execute(&mut m)?;

        assert_eq!(m.pc, 0x123);

        Ok(())
    }

    #[test]
    fn instruction_execute_load_vx_value() -> Result<()> {
        let mut m = Chip8::default();

        Instruction::LoadVxValue { vx: 5, value: 0x32 }.execute(&mut m)?;

        assert_eq!(m.v[5], 0x32);

        Ok(())
    }

    #[test]
    fn instruction_execute_add_vx_value() -> Result<()> {
        let mut m = Chip8::default();
        m.v[4] = 1;

        Instruction::AddVxValue { vx: 4, value: 0x33 }.execute(&mut m)?;

        assert_eq!(m.v[4], 0x34);

        Ok(())
    }

    #[test]
    fn instruction_execute_load_i_value() -> Result<()> {
        let mut m = Chip8::default();

        Instruction::LoadIValue { value: 0x123 }.execute(&mut m)?;

        assert_eq!(m.i, 0x123);

        Ok(())
    }

    #[test]
    fn instruction_execute_display_draw() -> Result<()> {
        let mut m = Chip8::default();
        m.i = 0;
        m.ram[0] = 0b10111111;
        m.ram[1] = 0b01001001;
        m.v[4] = 1;
        m.v[6] = 2;
        m.vram[2][1] = true;
        m.vram[2][2] = true;
        m.vram[2][3] = true;
        m.vram[3][1] = true;
        m.vram[3][2] = true;
        m.vram[3][3] = true;

        Instruction::DisplayDraw {
            vx: 4,
            vy: 6,
            height: 2,
        }
        .execute(&mut m)?;

        assert_eq!(m.vram[2][1], false);
        assert_eq!(m.vram[2][2], true);
        assert_eq!(m.vram[2][3], false);
        assert_eq!(m.vram[2][4], true);
        assert_eq!(m.vram[2][5], true);
        assert_eq!(m.vram[2][6], true);
        assert_eq!(m.vram[2][7], true);
        assert_eq!(m.vram[2][8], true);
        assert_eq!(m.vram[3][1], true);
        assert_eq!(m.vram[3][2], false);
        assert_eq!(m.vram[3][3], true);
        assert_eq!(m.vram[3][4], false);
        assert_eq!(m.vram[3][5], true);
        assert_eq!(m.vram[3][6], false);
        assert_eq!(m.vram[3][7], false);
        assert_eq!(m.vram[3][8], true);
        assert_eq!(m.v[0xF], 1);

        Ok(())
    }
}
