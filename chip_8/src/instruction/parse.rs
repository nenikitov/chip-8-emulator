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
    System {
        address: u16,
    },
    /// Clear the display (VRAM).
    DisplayClear,
    /// Jump to an instruction at address.
    Jump {
        address: u16,
    },
    /// Load a value into register Vx.
    SetVxWithValue {
        vx: usize,
        value: u8,
    },
    /// Add a value to register Vx.
    AddVxValue {
        vx: usize,
        value: u8,
    },
    /// Load a value into register I.
    SetIWithValue {
        value: u16,
    },
    /// Display a sprite from register I with specified height in the coordinates from registers Vx and Vy.
    DisplayDraw {
        vx: usize,
        vy: usize,
        height: u8,
    },
    /// Return from a subroutine.
    SubroutineReturn,
    /// Call a subroutine at address.
    SubroutineCall {
        address: u16,
    },
    // Skip the next instruction if value in a register Vx equals to a given value.
    SkipIfVxEquals {
        vx: usize,
        value: u8,
    },
    // Skip the next instruction if value in a register Vx does not equal to a given value.
    SkipIfVxNotEquals {
        vx: usize,
        value: u8,
    },
    // Skip the next instruction if value in a register Vx equals to value in register Vy.
    SkipIfVxEqualsVy {
        vx: usize,
        vy: usize,
    },
    // Skip the next instruction if value in a register Vx does not equal to value in register Vy.
    SkipIfVxNotEqualsVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx from Vy.
    SetVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx bitwise OR between Vx and Vy.
    OrVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx bitwise AND between Vx and Vy.
    AndVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx bitwise XOR between Vx and Vy.
    XorVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx the sum Vx and Vy and set the carry flag.
    AddVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx the difference Vx between and Vy and set the carry flag.
    SubtractVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Load a value into register Vx the difference between Vy and Vx and set the carry flag.
    SubtractVyWithVx {
        vx: usize,
        vy: usize,
    },
    /// Shift a value in a register Vx by 1 to the right and store the shifted out bit.
    /// **COMPATIBILITY:** Optionally copies Vy to Vx before shift.
    Shift1RightVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Shift a value in a register Vx by 1 to the left and store the shifted out bit.
    /// **COMPATIBILITY:** Optionally copies Vy to Vx before shift.
    Shift1LeftVxWithVy {
        vx: usize,
        vy: usize,
    },
    /// Jump to the offset + what is stored in V0.
    /// **COMPATIBILITY:** Optionally use Vx instead of V0.
    JumpWithOffset {
        vx: usize,
        value: u16,
    },
}

impl TryFrom<Opcode> for Instruction {
    type Error = ParseError;

    fn try_from(value: Opcode) -> Result<Self, Self::Error> {
        let (i, x, y, n, nn, nnn) = value.into();

        let instruction = match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::DisplayClear,
            (0x0, 0x0, 0xE, 0xE) => Instruction::SubroutineReturn,
            (0x0, _, _, _) => Instruction::System { address: nnn },
            (0x1, _, _, _) => Instruction::Jump { address: nnn },
            (0x2, _, _, _) => Instruction::SubroutineCall { address: nnn },
            (0x3, _, _, _) => Instruction::SkipIfVxEquals { vx: x, value: nn },
            (0x4, _, _, _) => Instruction::SkipIfVxNotEquals { vx: x, value: nn },
            (0x5, _, _, 0x0) => Instruction::SkipIfVxEqualsVy { vx: x, vy: y },
            (0x6, _, _, _) => Instruction::SetVxWithValue { vx: x, value: nn },
            (0x7, _, _, _) => Instruction::AddVxValue { vx: x, value: nn },
            (0x8, _, _, 0x0) => Instruction::SetVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x1) => Instruction::OrVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x2) => Instruction::AndVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x3) => Instruction::XorVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x4) => Instruction::AddVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x5) => Instruction::SubtractVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x6) => Instruction::Shift1RightVxWithVy { vx: x, vy: y },
            (0x8, _, _, 0x7) => Instruction::SubtractVyWithVx { vx: x, vy: y },
            (0x8, _, _, 0xE) => Instruction::Shift1LeftVxWithVy { vx: x, vy: y },
            (0x9, _, _, 0x0) => Instruction::SkipIfVxNotEqualsVy { vx: x, vy: y },
            (0xA, _, _, _) => Instruction::SetIWithValue { value: nnn },
            (0xB, _, _, _) => Instruction::JumpWithOffset { vx: x, value: nnn },
            (0xD, _, _, _) => Instruction::DisplayDraw {
                vx: x,
                vy: y,
                height: n as u8,
            },
            _ => return Err(ParseError::UnknownOpcode(value)),
        };

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn from_opcode_00e0_returns_display_clear() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x00E0)),
            Ok(Instruction::DisplayClear)
        );

        Ok(())
    }

    #[test]
    fn from_opcode_00ee_returns_subroutine_return() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x00EE)),
            Ok(Instruction::SubroutineReturn)
        );

        Ok(())
    }

    #[test]
    fn from_opcode_0nnn_returns_system() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x0123)),
            Ok(Instruction::System { address: 0x123 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_1nnn_returns_jump() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x1123)),
            Ok(Instruction::Jump { address: 0x123 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_2nnn_returns_subroutine_call() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x2123)),
            Ok(Instruction::SubroutineCall { address: 0x123 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_3xnn_returns_skip_if_vx_equals() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x3123)),
            Ok(Instruction::SkipIfVxEquals {
                vx: 0x1,
                value: 0x23
            })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_4xnn_returns_skip_if_vx_not_equals() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x4234)),
            Ok(Instruction::SkipIfVxNotEquals {
                vx: 0x2,
                value: 0x34
            })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_5xy0_returns_skip_if_vx_equals_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x5120)),
            Ok(Instruction::SkipIfVxEqualsVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_6xnn_returns_set_vx_with_value() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x6123)),
            Ok(Instruction::SetVxWithValue {
                vx: 0x1,
                value: 0x23
            })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_7xnn_returns_add_vx_value() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x7123)),
            Ok(Instruction::AddVxValue {
                vx: 0x1,
                value: 0x23
            })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy0_returns_set_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8120)),
            Ok(Instruction::SetVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy1_returns_or_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8121)),
            Ok(Instruction::OrVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy2_returns_and_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8122)),
            Ok(Instruction::AndVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy3_returns_xor_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8123)),
            Ok(Instruction::XorVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy4_returns_xor_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8124)),
            Ok(Instruction::AddVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy5_returns_subtract_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8125)),
            Ok(Instruction::SubtractVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy6_returns_shift_1_right_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8126)),
            Ok(Instruction::Shift1RightVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xy7_returns_subtract_vy_with_vx() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x8127)),
            Ok(Instruction::SubtractVyWithVx { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_8xye_returns_shift_1_left_vx_with_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x812E)),
            Ok(Instruction::Shift1LeftVxWithVy { vx: 0x1, vy: 0x2 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_9xy0_returns_skip_if_vx_not_equals_vy() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x9230)),
            Ok(Instruction::SkipIfVxNotEqualsVy { vx: 0x2, vy: 0x3 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_axnn_returns_set_i_with_value() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0xA123)),
            Ok(Instruction::SetIWithValue { value: 0x123 })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_bnnn_returns_jump_with_offset() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0xB123)),
            Ok(Instruction::JumpWithOffset {
                vx: 0x1,
                value: 0x123
            })
        );

        Ok(())
    }

    #[test]
    fn from_opcode_dxyn_returns_display_draw() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0xD123)),
            Ok(Instruction::DisplayDraw {
                vx: 0x1,
                vy: 0x2,
                height: 0x3
            })
        );

        Ok(())
    }
}
