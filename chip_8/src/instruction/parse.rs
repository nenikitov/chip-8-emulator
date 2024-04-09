use super::*;
use thiserror::Error;

/// Errors encountered during parsing of an opcode.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("opcode {0:?} is unknown")]
    UnknownOpcode(Opcode),
}

/// CPU instruction with required arguments.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    /// Clear the display (VRAM).
    ///
    /// * Opcode: `00E0`
    /// * Mnemonic: `CLS`
    DisplayClear,
    /// Return from a subroutine.
    ///
    /// * Opcode: `00EE`
    /// * Mnemonic: `RET`
    SubroutineReturn,
    /// Execute machine code routine at address.
    /// **WARNING:** Is unsupported.
    ///
    /// * Opcode: `0nnn`
    /// * Mnemonic: `SYS addr`
    System { address: u16 },
    /// Jump to an instruction at address.
    ///
    /// * Opcode: `1nnn`
    /// * Mnemonic: `JP addr`
    Jump { address: u16 },
    /// Call a subroutine at address.
    ///
    /// * Opcode: `2nnn`
    /// * Mnemonic: `CALL addr`
    SubroutineCall { address: u16 },
    // Skip the next instruction if value in `Vx` equals to a given value.
    ///
    /// * Opcode: `3xnn`
    /// * Mnemonic: `SE Vx byte`
    SkipIfVxEqualsValue { vx: usize, value: u8 },
    // Skip the next instruction if value in `Vx` does not equal to a given value.
    ///
    /// * Opcode: `4xnn`
    /// * Mnemonic: `SNE Vx byte`
    SkipIfVxNotEqualsValue { vx: usize, value: u8 },
    // Skip the next instruction if value in `Vx` equals to value in `Vy`.
    ///
    /// * Opcode: `5xy0`
    /// * Mnemonic: `SE Vx Vy`
    SkipIfVxEqualsVy { vx: usize, vy: usize },
    /// Load a value into `Vx`.
    ///
    /// * Opcode: `6xnn`
    /// * Mnemonic: `LD Vx byte`
    SetVxWithValue { vx: usize, value: u8 },
    /// Add a value and store the result to `Vx`.
    ///
    /// * Opcode: `7xnn`
    /// * Mnemonic: `ADD Vx byte`
    AddVxValue { vx: usize, value: u8 },
    /// Load a value from `Vy` into `Vx`.
    ///
    /// * Opcode: `8xy0`
    /// * Mnemonic: `LD Vx Vy`
    SetVxWithVy { vx: usize, vy: usize },
    /// Compute bitwise OR between `Vx` and `Vy` and store in `Vx`.
    ///
    /// * Opcode: `8xy1`
    /// * Mnemonic: `OR Vx Vy`
    OrVxWithVy { vx: usize, vy: usize },
    /// Compute bitwise AND between `Vx` and `Vy` and store in `Vx`.
    ///
    /// * Opcode: `8xy2`
    /// * Mnemonic: `AND Vx Vy`
    AndVxWithVy { vx: usize, vy: usize },
    /// Compute bitwise XOR between `Vx` and `Vy` and store in `Vx`.
    ///
    /// * Opcode: `8xy3`
    /// * Mnemonic: `XOR Vx Vy`
    XorVxWithVy { vx: usize, vy: usize },
    /// Add `Vx` and `Vy` and store in `Vx`.
    /// Store carry flag into `VF`.
    ///
    /// * Opcode: `8xy4`
    /// * Mnemonic: `ADD Vx Vy`
    AddVxWithVy { vx: usize, vy: usize },
    /// Subtract `Vx` and `Vy` and store in `Vx`.
    /// Store the opposite of a carry flag into `VF`.
    ///
    /// * Opcode: `8xy5`
    /// * Mnemonic: `SUB Vx Vy`
    SubtractVxWithVy { vx: usize, vy: usize },
    /// Shift 1 bit to the right and store a value in `Vx`.
    /// Store shifted bit into `VF`.
    ///
    /// **COMPATIBILITY:** Optionally copies Vy to Vx before shift.
    ///
    /// * Opcode: `8xy5`
    /// * Mnemonic: `SHR Vx Vy`
    Shift1RightVxWithVy { vx: usize, vy: usize },
    /// Subtract `Vy` and `Vx` and store in `Vx`.
    /// Store the opposite of a carry flag into `VF`.
    ///
    /// * Opcode: `8xy7`
    /// * Mnemonic: `SUBN Vx Vy`
    SubtractVyWithVx { vx: usize, vy: usize },
    /// Shift 1 bit to the left and store a value in `Vx`.
    /// Store shifted bit into `VF`.
    ///
    /// **COMPATIBILITY:** Optionally copies Vy to Vx before shift.
    ///
    /// * Opcode: `8xyE`
    /// * Mnemonic: `SHL Vx Vy`
    Shift1LeftVxWithVy { vx: usize, vy: usize },
    // Skip the next instruction if value in a `Vx` does not equal to value in `Vy`.
    ///
    /// * Opcode: `9xy0`
    /// * Mnemonic: `SNE Vx Vy`
    SkipIfVxNotEqualsVy { vx: usize, vy: usize },
    /// Load a value into `I`.
    ///
    /// * Opcode: `Annn`
    /// * Mnemonic: `LD I addr`
    SetIWithValue { value: u16 },
    /// Jump to the offset + value in `V0`.
    ///
    /// **COMPATIBILITY:** Optionally use `Vx` instead of `V0`.
    ///
    /// * Opcode: `Bnnn`
    /// * Mnemonic: `JP V0 + addr` or `JP Vx + addr`
    JumpWithOffset { vx: usize, address: u16 },
    /// Generate a random value, perform bitwise AND with a given value and put into `Vx`.
    ///
    /// * Opcode: `Cxnn`
    /// * Mnemonic: `RND Vx byte`
    SetVxWithRandom { vx: usize, value: u8 },
    /// Display a sprite from `I` with specified height in the coordinates from `Vx` and `Vy`.
    ///
    /// * Opcode: `Dxyn`
    /// * Mnemonic: `DRW Vx Vy height`
    DisplayDraw { vx: usize, vy: usize, height: u8 },
    /// Skip the next instruction if a key stored in `Vx` is pressed.
    ///
    /// * Opcode: `Ex9E`
    /// * Mnemonic: `SKP Vx`
    SkipIfVxKeyPressed { vx: usize },
    /// Skip the next instruction if a key stored in `Vx` is not pressed.
    ///
    /// * Opcode: `ExA1`
    /// * Mnemonic: `SKNP Vx`
    SkipIfVxKeyNotPressed { vx: usize },
    /// Load a value from `DT` into `Vx`.
    ///
    /// * Opcode: `Fx07`
    /// * Mnemonic: `LD Vx DT`
    SetVxWithDt { vx: usize },
    /// Stop execution and wait until a key is pressed.
    /// A key that was pressed is stored in `Vx`.
    ///
    /// * Opcode: `Fx0A`
    /// * Mnemonic: `LD Vx key`
    SetVxWithNextPressedKeyBlocking { vx: usize },
    /// Load a value from `Vx` into `DT`.
    ///
    /// * Opcode: `Fx15`
    /// * Mnemonic: `LD DT Vx`
    SetDtWithVx { vx: usize },
    /// Load a value from `Vx` into `ST`.
    ///
    /// * Opcode: `Fx18`
    /// * Mnemonic: `LD ST Vx`
    SetStWithVx { vx: usize },
    /// Add `I` and `Vx` and store in `I`.
    ///
    /// **COMPATIBILITY:** Optionally stores if resulting memory is outside in `VF`.
    ///
    /// * Opcode: `Fx1E`
    /// * Mnemonic: `ADD I Vx`
    AddIWithVx { vx: usize },
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
            (0x3, _, _, _) => Instruction::SkipIfVxEqualsValue { vx: x, value: nn },
            (0x4, _, _, _) => Instruction::SkipIfVxNotEqualsValue { vx: x, value: nn },
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
            (0xB, _, _, _) => Instruction::JumpWithOffset {
                vx: x,
                address: nnn,
            },
            (0xC, _, _, _) => Instruction::SetVxWithRandom { vx: x, value: nn },
            (0xD, _, _, _) => Instruction::DisplayDraw {
                vx: x,
                vy: y,
                height: n as u8,
            },
            (0xE, _, 0x9, 0xE) => Instruction::SkipIfVxKeyPressed { vx: x },
            (0xE, _, 0xA, 0x1) => Instruction::SkipIfVxKeyNotPressed { vx: x },
            (0xF, _, 0x0, 0x7) => Instruction::SetVxWithDt { vx: x },
            (0xF, _, 0x0, 0xA) => Instruction::SetVxWithNextPressedKeyBlocking { vx: x },
            (0xF, _, 0x1, 0x5) => Instruction::SetDtWithVx { vx: x },
            (0xF, _, 0x1, 0x8) => Instruction::SetStWithVx { vx: x },
            (0xF, _, 0x1, 0xE) => Instruction::AddIWithVx { vx: x },
            _ => return Err(ParseError::UnknownOpcode(value)),
        };

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use eyre::Result;
    use rstest::*;
    use similar_asserts::assert_eq;

    macro_rules! opcode {
        { i: $i:expr, x: $x:expr, y: $y:expr, n: $n:expr } => (
            ($i << 12) as u16 + ($x << 8) as u16 + ($y << 4) as u16 + ($n as u16)
        );
        { i: $i:expr, x: $x:expr, nn: $nn: expr } => (
            ($i << 12) as u16 + ($x << 8) as u16 + ($nn as u16)
        );
        { i: $i:expr, nnn: $nnn: expr } => (
            ($i << 12) as u16 + ($nnn as u16)
        );
    }

    #[rstest]
    fn from_opcode_00e0_returns_display_clear() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x00E0)),
            Ok(Instruction::DisplayClear)
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_00ee_returns_subroutine_return() -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(0x00EE)),
            Ok(Instruction::SubroutineReturn)
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_0nnn_returns_system(#[values(0x123, 0x234)] address: u16) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x0, nnn: address })),
            Ok(Instruction::System { address })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_1nnn_returns_jump(#[values(0x123, 0x234)] address: u16) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x1, nnn: address })),
            Ok(Instruction::Jump { address })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_2nnn_returns_subroutine_call(
        #[values(0x123, 0x234)] address: u16,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x2, nnn: address })),
            Ok(Instruction::SubroutineCall { address })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_3xnn_returns_skip_if_vx_equals_value(
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x3, x: vx, nn: value })),
            Ok(Instruction::SkipIfVxEqualsValue { vx, value })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_4xnn_returns_skip_if_vx_not_equals_value(
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x4, x: vx, nn: value })),
            Ok(Instruction::SkipIfVxNotEqualsValue { vx, value })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_5xy0_returns_skip_if_vx_equals_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x5, x: vx, y: vy, n: 0x0 })),
            Ok(Instruction::SkipIfVxEqualsVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_6xnn_returns_set_vx_with_value(
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x6, x: vx, nn: value })),
            Ok(Instruction::SetVxWithValue { vx, value })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_7xnn_returns_add_vx_value(
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x7, x: vx, nn: value })),
            Ok(Instruction::AddVxValue { vx, value })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy0_returns_set_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x0 })),
            Ok(Instruction::SetVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy1_returns_or_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x1 })),
            Ok(Instruction::OrVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy2_returns_and_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x2 })),
            Ok(Instruction::AndVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy3_returns_xor_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x3 })),
            Ok(Instruction::XorVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy4_returns_xor_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x4 })),
            Ok(Instruction::AddVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy5_returns_subtract_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x5 })),
            Ok(Instruction::SubtractVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy6_returns_shift_1_right_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x6 })),
            Ok(Instruction::Shift1RightVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xy7_returns_subtract_vy_with_vx(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0x7 })),
            Ok(Instruction::SubtractVyWithVx { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_8xye_returns_shift_1_left_vx_with_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x8, x: vx, y: vy, n: 0xE })),
            Ok(Instruction::Shift1LeftVxWithVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_9xy0_returns_skip_if_vx_not_equals_vy(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0x9, x: vx, y: vy, n: 0x0 })),
            Ok(Instruction::SkipIfVxNotEqualsVy { vx, vy })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_annn_returns_set_i_with_value(#[values(0x123, 0x234)] value: u16) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xA, nnn: value })),
            Ok(Instruction::SetIWithValue { value })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_bnnn_returns_jump_with_offset(
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xB, x: vx, nn: value })),
            Ok(Instruction::JumpWithOffset {
                vx,
                address: (vx << 8) as u16 + value as u16
            })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_cxnn_returns_set_vx_with_random(
        #[values(1, 2)] vx: usize,
        #[values(0x12, 0x23)] value: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xC, x: vx, nn: value })),
            Ok(Instruction::SetVxWithRandom { vx, value })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_dxyn_returns_display_draw(
        #[values(1, 2)] vx: usize,
        #[values(2, 3)] vy: usize,
        #[values(4, 5)] height: u8,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xD, x: vx, y: vy, n: height })),
            Ok(Instruction::DisplayDraw { vx, vy, height })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_ex9e_returns_skip_if_vx_key_pressed(#[values(1, 2)] vx: usize) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xE, x: vx, nn: 0x9E })),
            Ok(Instruction::SkipIfVxKeyPressed { vx })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_exa1_returns_skip_if_vx_key_not_pressed(
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xE, x: vx, nn: 0xA1 })),
            Ok(Instruction::SkipIfVxKeyNotPressed { vx })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_fx07_returns_set_vx_with_dt(#[values(1, 2)] vx: usize) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xF, x: vx, nn: 0x07 })),
            Ok(Instruction::SetVxWithDt { vx })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_fx0a_returns_set_vx_with_next_pressed_key_blocking(
        #[values(1, 2)] vx: usize,
    ) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xF, x: vx, nn: 0x0A })),
            Ok(Instruction::SetVxWithNextPressedKeyBlocking { vx })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_fx15_returns_set_dt_with_vx(#[values(1, 2)] vx: usize) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xF, x: vx, nn: 0x15 })),
            Ok(Instruction::SetDtWithVx { vx })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_fx18_returns_set_st_with_vx(#[values(1, 2)] vx: usize) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xF, x: vx, nn: 0x18 })),
            Ok(Instruction::SetStWithVx { vx })
        );
        Ok(())
    }

    #[rstest]
    fn from_opcode_fx1e_returns_add_i_with_vx(#[values(1, 2)] vx: usize) -> Result<()> {
        assert_eq!(
            Instruction::try_from(Opcode::from(opcode! { i: 0xF, x: vx, nn: 0x1E })),
            Ok(Instruction::AddIWithVx { vx })
        );
        Ok(())
    }
}
