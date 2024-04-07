#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Instruction bytes split into multiple parts.
pub struct Opcode {
    /// First nibble.
    i: usize,
    /// Second nibble.
    x: usize,
    /// Third nibble.
    y: usize,
    /// Forth nibble.
    n: usize,
    /// Last byte.
    nn: u8,
    /// Last 12-bit word.
    nnn: u16,
}

impl From<u16> for Opcode {
    fn from(ins: u16) -> Self {
        let i = (ins & 0xF000) >> 12;
        let x = (ins & 0x0F00) >> 8;
        let y = (ins & 0x00F0) >> 4;
        let n = ins & 0x000F;
        let nn = ins & 0x00FF;
        let nnn = ins & 0x0FFF;

        Self {
            i: i as usize,
            x: x as usize,
            y: y as usize,
            n: n as usize,
            nn: nn as u8,
            nnn,
        }
    }
}

impl From<(u8, u8)> for Opcode {
    fn from((a, b): (u8, u8)) -> Self {
        let i = ((a as u16) << 8) | (b as u16);
        Self::from(i)
    }
}

impl From<Opcode> for (usize, usize, usize, usize, u8, u16) {
    fn from(value: Opcode) -> Self {
        (value.i, value.x, value.y, value.n, value.nn, value.nnn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use eyre::Result;
    use rstest::*;
    use similar_asserts::assert_eq;

    #[rstest]
    fn from_u16_splits(
        #[values(
            (0xD123, 0xD, 0x1, 0x2, 0x3, 0x23, 0x123),
            (0xA974, 0xA, 0x9, 0x7, 0x4, 0x74, 0x974)
        )]
        opcode: (u16, usize, usize, usize, usize, u8, u16),
    ) -> Result<()> {
        let (opcode, i, x, y, n, nn, nnn) = opcode;

        assert_eq!(
            Opcode::from(opcode),
            Opcode {
                i,
                x,
                y,
                n,
                nn,
                nnn
            }
        );

        Ok(())
    }

    #[rstest]
    fn from_u8_u8_splits(
        #[values(
            ((0xD1, 0x23), 0xD, 0x1, 0x2, 0x3, 0x23, 0x123),
            ((0xA9, 0x74), 0xA, 0x9, 0x7, 0x4, 0x74, 0x974)
        )]
        opcode: ((u8, u8), usize, usize, usize, usize, u8, u16),
    ) -> Result<()> {
        let (opcode, i, x, y, n, nn, nnn) = opcode;

        assert_eq!(
            Opcode::from(opcode),
            Opcode {
                i,
                x,
                y,
                n,
                nn,
                nnn,
            }
        );

        Ok(())
    }

    #[rstest]
    fn into_tuple_converts(
        #[values(
            (0xD123, 0xD, 0x1, 0x2, 0x3, 0x23, 0x123),
            (0xA974, 0xA, 0x9, 0x7, 0x4, 0x74, 0x974)
        )]
        opcode: (u16, usize, usize, usize, usize, u8, u16),
    ) -> Result<()> {
        let (opcode, i_target, x_target, y_target, n_target, nn_target, nnn_target) = opcode;

        let (i, x, y, n, nn, nnn) = Opcode::from(opcode).into();
        assert_eq!(
            (i, x, y, n, nn, nnn),
            (i_target, x_target, y_target, n_target, nn_target, nnn_target)
        );

        Ok(())
    }
}
