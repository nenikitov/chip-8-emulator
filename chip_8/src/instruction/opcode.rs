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
    nn: u16,
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
            nn,
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

impl Into<(usize, usize, usize, usize, u16, u16)> for Opcode {
    fn into(self) -> (usize, usize, usize, usize, u16, u16) {
        (self.i, self.x, self.y, self.n, self.nn, self.nnn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u16_splits() {
        assert_eq!(
            Opcode::from(0xD123),
            Opcode {
                i: 0xD,
                x: 0x1,
                y: 0x2,
                n: 0x3,
                nn: 0x23,
                nnn: 0x123
            }
        );
    }

    #[test]
    fn from_u8_splits() {
        assert_eq!(
            Opcode::from((0xA9, 0x74)),
            Opcode {
                i: 0xA,
                x: 0x9,
                y: 0x7,
                n: 0x4,
                nn: 0x74,
                nnn: 0x974
            }
        );
    }

    #[test]
    fn into_tuple_converts() {
        let (i, x, y, n, nn, nnn) = Opcode::from(0xA2B5).into();
        assert_eq!((i, x, y, n, nn, nnn), (0xA, 0x2, 0xB, 0x5, 0xB5, 0x2B5))
    }
}
