const FONT: [[u8; 5]; 16] = [
    [
        0b11110000, // ####
        0b10010000, // #  #
        0b10010000, // #  #
        0b10010000, // #  #
        0b11110000, // ####
    ],
    [
        0b00100000, //   #
        0b01100000, //  ##
        0b00100000, //   #
        0b00100000, //   #
        0b01110000, //  ###
    ],
    [
        0b11110000, // ####
        0b00010000, //    #
        0b11110000, // ####
        0b10000000, // #
        0b11110000, // ####
    ],
    [
        0b11110000, // ####
        0b00010000, //    #
        0b11110000, // ####
        0b00010000, //    #
        0b11110000, // ####
    ],
    [
        0b10010000, // #  #
        0b10010000, // #  #
        0b11110000, // ####
        0b00010000, //    #
        0b00010000, //    #
    ],
    [
        0b11110000, // ####
        0b10000000, // #
        0b11110000, // ####
        0b00010000, //    #
        0b11110000, // ####
    ],
    [
        0b11110000, // ####
        0b10000000, // #
        0b11110000, // ####
        0b10010000, // #  #
        0b11110000, // ####
    ],
    // 7
    [
        0b11110000, // ####
        0b00010000, //    #
        0b00100000, //   #
        0b01000000, //  #
        0b01000000, //  #
    ],
    [
        0b11110000, // ####
        0b10010000, // #  #
        0b11110000, // ####
        0b10010000, // #  #
        0b11110000, // ####
    ],
    [
        0b11110000, // ####
        0b10010000, // #  #
        0b11110000, // ####
        0b00010000, //    #
        0b11110000, // ####
    ],
    [
        0b11110000, // ####
        0b10010000, // #  #
        0b11110000, // ####
        0b10010000, // #  #
        0b10010000, // #  #
    ],
    [
        0b11100000, // ###
        0b10010000, // #  #
        0b11100000, // ###
        0b10010000, // #  #
        0b11100000, // ###
    ],
    [
        0b11110000, // ####
        0b10000000, // #
        0b10000000, // #
        0b10000000, // #
        0b11110000, // ####
    ],
    [
        0b11100000, // ###
        0b10010000, // #  #
        0b10010000, // #  #
        0b10010000, // #  #
        0b11100000, // ###
    ],
    [
        0b11110000, // ####
        0b10000000, // #
        0b11110000, // ####
        0b10000000, // #
        0b11110000, // ####
    ],
    [
        0b11110000, // ####
        0b10000000, // #
        0b11110000, // ####
        0b10000000, // #
        0b10000000, // #
    ],
];

#[derive(Debug, PartialEq, Eq)]
pub struct Memory {
    /// RAM.
    /// * `0x000..=0x1FFF` is unused (except the font).
    /// * Font is stored in `0x50..=0x9F` by convention.
    /// * Programs are stored in `0x200..`.
    pub ram: [u8; Self::SIZE_RAM],
    /// Display buffer containing the state of each pixel.
    pub vram: [[bool; Self::SIZE_DISPLAY_WIDTH]; Self::SIZE_DISPLAY_HEIGHT],
    /// Indexes in RAM of current subroutines.
    pub stack: Vec<u16>,
    /// Index in RAM where current execution is.
    pub pc: u16,
    /// Timer to stop execution when non 0. Should decrement at 60Hz rate.
    pub dt: u8,
    /// Timer play beep when non 0. Should decrement at 60Hz rate.
    pub st: u8,
    /// Index register often used to store memory addresses.
    pub i: u16,
    /// General purpose registers.
    pub v: [u8; Self::SIZE_REGISTERS],
}

impl Memory {
    pub const SIZE_RAM: usize = 4 * 1024;
    pub const SIZE_REGISTERS: usize = 16;
    pub const SIZE_DISPLAY_WIDTH: usize = 64;
    pub const SIZE_DISPLAY_HEIGHT: usize = 32;
    pub const PROGRAM_START: u16 = 0x200;

    pub const INDEX_FLAG_REGISTER: usize = Self::SIZE_REGISTERS - 1;
}

impl Default for Memory {
    fn default() -> Self {
        let mut s = Self {
            ram: [0; Self::SIZE_RAM],
            vram: [[false; Self::SIZE_DISPLAY_WIDTH]; Self::SIZE_DISPLAY_HEIGHT],
            stack: Vec::default(),
            pc: Self::PROGRAM_START,
            dt: 0,
            st: 0,
            i: 0,
            v: [0; Self::SIZE_REGISTERS],
        };
        s.clear_memory();
        s
    }
}

impl Memory {
    /// Reset memory and load a ROM into RAM.
    ///
    /// # Arguments
    ///
    /// * `program` - Program to load.
    pub(crate) fn load(&mut self, rom: &[u8]) {
        self.clear_memory();
        self.ram[Self::PROGRAM_START as usize..][..rom.len()].copy_from_slice(rom);
    }

    /// Advance program counter to the next instruction.
    ///
    /// **NOTE:** Does not execute any instructions.
    pub(crate) fn increment_pc(&mut self) {
        self.pc += 2;
    }

    /// Perform an update of the timer.
    /// Should be called at a fixed rate of 60hz.
    pub(crate) fn advance_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    /// Reset display memory.
    pub(crate) fn clear_vram(&mut self) {
        self.vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = false));
    }

    /// Reset all memory and load font into RAM.
    fn clear_memory(&mut self) {
        self.ram.iter_mut().for_each(|e| *e = 0);
        self.ram[0x50..=0x9F].copy_from_slice(FONT.flatten());
        self.clear_vram();
        self.stack.clear();
        self.v.iter_mut().for_each(|e| *e = 0);
        self.pc = Self::PROGRAM_START;
        self.dt = 0;
        self.st = 0;
        self.i = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use eyre::Result;

    #[test]
    fn default_initializes_ram() -> Result<()> {
        let m = Memory::default();
        assert_eq!(m.ram[0..0x50], [0; 0x50]);
        assert_eq!(&m.ram[0x50..=0x9F], FONT.flatten());
        assert_eq!(m.ram[0xA0..Memory::SIZE_RAM], [0; Memory::SIZE_RAM - 0xA0]);

        Ok(())
    }
    #[test]
    fn default_initializes_display() -> Result<()> {
        let m = Memory::default();
        assert_eq!(
            m.vram,
            [[false; Memory::SIZE_DISPLAY_WIDTH]; Memory::SIZE_DISPLAY_HEIGHT]
        );

        Ok(())
    }
    #[test]
    fn default_initializes_stack() -> Result<()> {
        let m = Memory::default();
        assert_eq!(m.stack, [0; 0]);

        Ok(())
    }
    #[test]
    fn default_initializes_regitsers() -> Result<()> {
        let m = Memory::default();
        assert_eq!(m.i, 0);
        assert_eq!(m.v, [0; Memory::SIZE_REGISTERS]);

        Ok(())
    }
    #[test]
    fn default_initializes_other() -> Result<()> {
        let m = Memory::default();
        assert_eq!(m.pc, Memory::PROGRAM_START);
        assert_eq!(m.dt, 0);
        assert_eq!(m.st, 0);

        Ok(())
    }

    #[test]
    fn load_loads() -> Result<()> {
        let mut m = Memory::default();

        m.load(&[10, 20, 30]);

        assert_eq!(m.ram[Memory::PROGRAM_START as usize..][..3], [10, 20, 30]);

        Ok(())
    }

    #[test]
    fn load_resets_memory() -> Result<()> {
        let empty = Memory::default();
        let mut modified = Memory::default();

        modified.ram[Memory::PROGRAM_START as usize + 10] = 10;
        modified.pc = 20;
        modified.vram[0][0] = true;

        modified.load(&[]);

        assert_eq!(modified, empty);

        Ok(())
    }

    #[test]
    fn increment_pc_increments() -> Result<()> {
        let mut m = Memory::default();

        m.increment_pc();
        m.increment_pc();
        m.increment_pc();

        assert_eq!(m.pc, Memory::PROGRAM_START + 6);

        Ok(())
    }

    #[test]
    fn advance_timer_decrements() -> Result<()> {
        let mut m = Memory::default();

        m.dt = 10;
        m.st = 13;

        m.advance_timer();

        assert_eq!(m.dt, 9);
        assert_eq!(m.st, 12);

        Ok(())
    }

    #[test]
    fn advance_timer_doesnt_underflow_if_0() -> Result<()> {
        let mut m = Memory::default();

        m.dt = 0;
        m.st = 0;

        m.advance_timer();

        assert_eq!(m.dt, 0);
        assert_eq!(m.st, 0);

        Ok(())
    }

    #[test]
    fn clear_vram_resets() -> Result<()> {
        let empty = Memory::default();
        let mut modified = Memory::default();
        modified
            .vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = true));
        modified.clear_vram();

        assert_eq!(modified, empty);

        Ok(())
    }

    #[test]
    fn clear_vram_doesnt_reset_others() -> Result<()> {
        let mut m = Memory::default();
        m.ram[500] = 50;
        m.clear_vram();

        assert_eq!(m.ram[500], 50);

        Ok(())
    }

    #[test]
    fn clear_works() -> Result<()> {
        let empty = Memory::default();
        let mut modified = Memory::default();
        modified.ram[Memory::PROGRAM_START as usize] = 0xFF;
        modified
            .vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = true));
        modified.stack.push(0xFF);
        modified.pc += 2;
        modified.dt = 10;
        modified.st = 20;
        modified.i = 1;
        modified.v.iter_mut().for_each(|e| *e = 5);
        modified.clear_memory();

        assert_eq!(modified, empty);

        Ok(())
    }
}
