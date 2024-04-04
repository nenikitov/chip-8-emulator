use super::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("opcode {0:?} is unknown")]
    UnknownOpcode(Opcode),
}

/// CPU instruction with required arguments.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    /// Execute machine code routine at address.
    System { address: u16 },
    /// Clear the display (VRAM).
    DisplayClear,
    /// Jump to an instruction at address.
    Jump { address: u16 },
    /// Load a value into register Vx.
    LoadVxValue { vx: usize, value: u16 },
    /// Add a value to register Vx.
    AddVxValue { vx: usize, value: u16 },
    /// Load a value into register I.
    LoadIValue { value: u16 },
    /// Display a sprite from register I with specified height in the coordinates from registers Vx and Vy.
    DisplayDraw { vx: usize, vy: usize, height: u16 },
}

impl TryFrom<Opcode> for Instruction {
    type Error = ParseError;

    fn try_from(value: Opcode) -> Result<Self, Self::Error> {
        let (i, x, y, n, nn, nnn) = value.into();

        let instruction = match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::DisplayClear,
            (0x0, _, _, _) => Instruction::System { address: nnn },
            (0x1, _, _, _) => Instruction::Jump { address: nnn },
            (0x6, _, _, _) => Instruction::LoadVxValue { vx: x, value: nn },
            (0x7, _, _, _) => Instruction::AddVxValue { vx: x, value: nn },
            (0xA, _, _, _) => Instruction::LoadIValue { value: nnn },
            (0xD, _, _, _) => Instruction::DisplayDraw {
                vx: x,
                vy: y,
                height: n as u16,
            },
            _ => return Err(ParseError::UnknownOpcode(value)),
        };

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_opcode_00e0_returns_display_clear() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x00E0)),
            Ok(Instruction::DisplayClear)
        )
    }

    #[test]
    fn from_opcode_0nnn_returns_system() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x0123)),
            Ok(Instruction::System { address: 0x123 })
        )
    }

    #[test]
    fn from_opcode_1nnn_returns_jump() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x1123)),
            Ok(Instruction::Jump { address: 0x123 })
        )
    }

    #[test]
    fn from_opcode_6xnn_returns_load_vx_value() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x6123)),
            Ok(Instruction::LoadVxValue {
                vx: 0x1,
                value: 0x23
            })
        )
    }

    #[test]
    fn from_opcode_7xnn_returns_add_vx_value() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x7123)),
            Ok(Instruction::AddVxValue {
                vx: 0x1,
                value: 0x23
            })
        )
    }

    #[test]
    fn from_opcode_axnn_returns_load_i_value() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0xA123)),
            Ok(Instruction::LoadIValue { value: 0x123 })
        )
    }

    #[test]
    fn from_opcode_dxyn_returns_display_draw() {
        assert_eq!(
            Instruction::try_from(Opcode::from(0xD123)),
            Ok(Instruction::DisplayDraw {
                vx: 0x1,
                vy: 0x2,
                height: 0x3
            })
        )
    }
}
