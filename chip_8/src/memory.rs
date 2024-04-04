use crate::instruction::{ExecuteOnMemory, Instruction, Opcode};

const FONT: [u8; 16 * 5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const SIZE_RAM: usize = 4 * 1024;
const SIZE_REGISTERS: usize = 16;
pub const SIZE_DISPLAY: (usize, usize) = (64, 32);
pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug, PartialEq)]
/// Chip 8 memory.
pub struct Memory {
    /// RAM.
    /// * `0x000..=0x1FFF` is unused (except the font).
    /// * Font is stored in `0x50..=0x9F` by convention.
    /// * Programs are stored in `0x200..`.
    pub ram: [u8; SIZE_RAM],
    /// Display buffer containing the state of each pixel.
    pub vram: [[bool; SIZE_DISPLAY.0]; SIZE_DISPLAY.1],
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
    pub v: [u16; 16],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self {
            ram: [0; SIZE_RAM],
            vram: [[false; SIZE_DISPLAY.0]; SIZE_DISPLAY.1],
            stack: Vec::new(),
            pc: PROGRAM_START,
            dt: 0,
            st: 0,
            i: 0,
            v: [0; SIZE_REGISTERS],
        };
        memory.clear();
        memory
    }

    /// Perform a next instruction.
    /// Should be called at around 500 - 1000 hz.
    pub fn advance_instruction(&mut self) {
        let opcode = Opcode::from((self.ram[self.pc as usize], self.ram[self.pc as usize + 1]));
        self.pc += 2;
        Instruction::from(opcode).execute(self);
    }

    /// Perform an update of the timer.
    /// Should be called at a fixed rate of 60 hz.
    pub fn advance_timer(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    /// Reset all memory.
    pub fn clear(&mut self) {
        self.ram.iter_mut().for_each(|e| *e = 0);
        self.ram[0x50..=0x9F].copy_from_slice(&FONT);
        self.clear_vram();
        self.stack.clear();
        self.v.iter_mut().for_each(|e| *e = 0);
        self.pc = PROGRAM_START;
        self.dt = 0;
        self.st = 0;
        self.i = 0;
    }

    pub fn clear_vram(&mut self) {
        self.vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = false));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_initializes_ram() {
        let m = Memory::new();
        assert_eq!(m.ram[0..0x50], [0; 0x50]);
        assert_eq!(m.ram[0x50..=0x9F], FONT);
        assert_eq!(m.ram[0xA0..SIZE_RAM], [0; SIZE_RAM - 0xA0]);
    }
    #[test]
    fn new_initializes_display() {
        let m = Memory::new();
        assert_eq!(m.vram, [[false; SIZE_DISPLAY.0]; SIZE_DISPLAY.1]);
    }
    #[test]
    fn new_initializes_stack() {
        let m = Memory::new();
        assert_eq!(m.stack, [0; 0]);
    }
    #[test]
    fn new_initializes_regitsers() {
        let m = Memory::new();
        assert_eq!(m.i, 0);
        assert_eq!(m.v, [0; SIZE_REGISTERS]);
    }
    #[test]
    fn new_initializes_other() {
        let m = Memory::new();
        assert_eq!(m.pc, PROGRAM_START);
        assert_eq!(m.dt, 0);
        assert_eq!(m.st, 0);
    }

    #[test]
    fn clear_resets() {
        let empty = Memory::new();
        let mut modified = Memory::new();
        modified.ram[PROGRAM_START as usize] = 0xFF;
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
        modified.clear();
        assert_eq!(modified, empty);
    }

    #[test]
    fn clear_vram_resets() {
        let empty = Memory::new();
        let mut modified = Memory::new();
        modified
            .vram
            .iter_mut()
            .for_each(|e| e.iter_mut().for_each(|e| *e = true));
        modified.clear_vram();
        assert_eq!(modified, empty);
    }
}
