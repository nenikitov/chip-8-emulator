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

/// Memory available to CHIP-8.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Memory {
    /// RAM.
    ///
    /// * `0x000..=0x1FFF` is unused (except the font).
    /// * Font is stored in `0x50..=0x9F` by convention.
    /// * Programs are stored in `0x200..`.
    pub ram: [u8; Self::SIZE_RAM],
    /// Display buffer containing the state of each pixel.
    ///
    /// Stored in `[y][x]` format.
    pub vram: [[bool; Self::SIZE_DISPLAY_WIDTH]; Self::SIZE_DISPLAY_HEIGHT],
    /// Indexes in RAM of current subroutines.
    pub stack: Vec<u16>,
    /// Index in RAM where current execution is.
    pub pc: u16,
    /// Timer to stop execution when non 0.
    // Should decrement at 60Hz rate.
    pub dt: u8,
    /// Timer play beep when non 0.
    /// Should decrement at 60Hz rate.
    pub st: u8,
    /// Index register often used to store memory addresses.
    pub i: u16,
    /// General purpose registers.
    pub v: [u8; Self::SIZE_REGISTERS],
    /// If the keys are pressed.
    pub keys: [bool; Self::SIZE_KEYS],
}

impl Memory {
    pub const SIZE_RAM: usize = 4 * 1024;
    pub const SIZE_REGISTERS: usize = 16;
    pub const SIZE_KEYS: usize = 16;
    pub const SIZE_DISPLAY_WIDTH: usize = 64;
    pub const SIZE_DISPLAY_HEIGHT: usize = 32;

    pub const INDEX_PROGRAM_START: u16 = 0x200;

    pub const INDEX_FONT_START: usize = 0x50;
    pub const INDEX_FLAG_REGISTER: usize = Self::SIZE_REGISTERS - 1;
}

impl Default for Memory {
    fn default() -> Self {
        let mut s = Self {
            ram: [0; Self::SIZE_RAM],
            vram: [[false; Self::SIZE_DISPLAY_WIDTH]; Self::SIZE_DISPLAY_HEIGHT],
            stack: Vec::default(),
            pc: Self::INDEX_PROGRAM_START,
            dt: 0,
            st: 0,
            i: 0,
            v: [0; Self::SIZE_REGISTERS],
            keys: [false; Self::SIZE_KEYS],
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
        self.ram[Self::INDEX_PROGRAM_START as usize..][..rom.len()].copy_from_slice(rom);
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
        self.ram[Memory::INDEX_FONT_START..][..16 * 5].copy_from_slice(FONT.flatten());
        self.clear_vram();
        self.stack.clear();
        self.v.iter_mut().for_each(|e| *e = 0);
        self.pc = Self::INDEX_PROGRAM_START;
        self.dt = 0;
        self.st = 0;
        self.i = 0;
        self.keys = [false; Self::SIZE_KEYS];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use eyre::Result;
    use rstest::*;
    use similar_asserts::assert_eq;

    #[fixture]
    fn target() -> Memory {
        let mut memory = Memory::default();

        memory.ram[Memory::INDEX_PROGRAM_START as usize..][..4].copy_from_slice(&[
            0x61, 0x02, // Load 2 into register 1
            0x71, 0x03, // Add 3 to it
        ]);
        memory.vram[0].iter_mut().for_each(|e| *e = true);
        memory.stack.push(Memory::INDEX_PROGRAM_START);
        memory.dt = 60;
        memory.st = 10;
        memory.i = 100;
        memory.v = [0, 1, 2, 3, 4, 5, 31, 59, 0, 1, 2, 3, 4, 5, 30, 60];
        memory.keys = [
            true, false, true, false, true, false, true,
            false, // First 8 are pressed and unpressed
            false, false, false, false, false, false, false, false, // Last 8 are not pressed
        ];

        memory
    }

    #[fixture]
    fn result(target: Memory) -> Memory {
        target.clone()
    }

    #[rstest]
    fn default_initializes_ram() -> Result<()> {
        let target = Memory::default();

        assert_eq!(
            target.ram[0..Memory::INDEX_FONT_START],
            [0; Memory::INDEX_FONT_START]
        );
        assert_eq!(
            &target.ram[Memory::INDEX_FONT_START..][..16 * 5],
            FONT.flatten()
        );
        assert_eq!(
            target.ram[0xA0..Memory::SIZE_RAM],
            [0; Memory::SIZE_RAM - 0xA0]
        );
        Ok(())
    }

    #[rstest]
    fn default_initializes_display() -> Result<()> {
        let target = Memory::default();

        assert_eq!(
            target.vram,
            [[false; Memory::SIZE_DISPLAY_WIDTH]; Memory::SIZE_DISPLAY_HEIGHT]
        );
        Ok(())
    }

    #[rstest]
    fn default_initializes_stack() -> Result<()> {
        let m = Memory::default();

        assert_eq!(m.stack, vec![]);
        Ok(())
    }

    #[rstest]
    fn default_initializes_regitsers() -> Result<()> {
        let m = Memory::default();

        assert_eq!(m.i, 0);
        assert_eq!(m.v, [0; Memory::SIZE_REGISTERS]);
        Ok(())
    }

    #[rstest]
    fn default_initializes_other() -> Result<()> {
        let m = Memory::default();

        assert_eq!(m.pc, Memory::INDEX_PROGRAM_START);
        assert_eq!(m.dt, 0);
        assert_eq!(m.st, 0);
        Ok(())
    }

    #[rstest]
    fn load_loads() -> Result<()> {
        let mut target = Memory::default();
        let mut result = Memory::default();

        target.load(&[10, 20, 30]);

        result.ram[Memory::INDEX_PROGRAM_START as usize..][..3].copy_from_slice(&[10, 20, 30]);

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn load_resets_memory(
        mut target: Memory,
        #[with(Memory::default())] mut result: Memory,
    ) -> Result<()> {
        target.load(&[]);

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn increment_pc_increments(mut target: Memory, mut result: Memory) -> Result<()> {
        for _ in 0..3 {
            target.increment_pc();
        }

        result.pc += 2 * 3;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn advance_timer_decrements(mut target: Memory, mut result: Memory) -> Result<()> {
        for _ in 0..3 {
            target.advance_timer();
        }

        result.dt -= 3;
        result.st -= 3;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn advance_timer_doesnt_underflow_if_0(mut target: Memory, mut result: Memory) -> Result<()> {
        for _ in 0..100 {
            target.advance_timer();
        }

        result.dt = 0;
        result.st = 0;

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn clear_vram_resets(mut target: Memory, mut result: Memory) -> Result<()> {
        target.clear_vram();

        result.vram = [[false; Memory::SIZE_DISPLAY_WIDTH]; Memory::SIZE_DISPLAY_HEIGHT];

        assert_eq!(target, result);
        Ok(())
    }

    #[rstest]
    fn clear_works(
        mut target: Memory,
        #[with(Memory::default())] mut result: Memory,
    ) -> Result<()> {
        target.clear_memory();

        assert_eq!(target, result);
        Ok(())
    }
}
