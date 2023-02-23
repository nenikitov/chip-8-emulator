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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

const SIZE_RAM: usize = 4 * 1024;
const SIZE_REGISTERS: usize = 16;
pub const SIZE_DISPLAY: (u16, u16) = (64, 32);
pub const SIZE_DISPLAY_TOTAL: usize = (SIZE_DISPLAY.0 * SIZE_DISPLAY.1) as usize;
pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Memory {
    pub ram: [u8; SIZE_RAM],
    pub vram: [bool; SIZE_DISPLAY_TOTAL],
    pub stack: Vec<u16>,
    pub pc: u16,
    pub dt: u8,
    pub st: u8,
    pub i: u16,
    pub v: [u16; 16]
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self {
            ram: [0; SIZE_RAM],
            vram: [false; SIZE_DISPLAY_TOTAL],
            stack: Vec::new(),
            pc: PROGRAM_START,
            dt: 0,
            st: 0,
            i: 0,
            v: [0; SIZE_REGISTERS]
        };
        memory.clear();
        memory
    }

    pub fn clear(&mut self) {
        self.ram.iter_mut().for_each(|e| *e = 0);
        self.ram[0x50..0xA0].copy_from_slice(&FONT);
        self.vram.iter_mut().for_each(|e| *e = false);
        self.stack.clear();
        self.v.iter_mut().for_each(|e| *e = 0);
        self.pc = PROGRAM_START;
        self.dt = 0;
        self.st = 0;
        self.i = 0;
    }
}


#[test]
fn memory_new_initializes_ram() {
    let m = Memory::new();
    assert_eq!(m.ram[0..0x50], [0; 0x50]);
    assert_eq!(m.ram[0x50..0xA0], FONT);
    assert_eq!(m.ram[0xA0..SIZE_RAM], [0; SIZE_RAM - 0xA0]);
}
#[test]
fn memory_new_initializes_display() {
    let m = Memory::new();
    assert_eq!(m.vram, [false; SIZE_DISPLAY_TOTAL]);
}
#[test]
fn memory_new_initializes_stack() {
    let m = Memory::new();
    assert_eq!(m.stack, [0; 0]);
}
#[test]
fn memory_new_initializes_regitsers() {
    let m = Memory::new();
    assert_eq!(m.i, 0);
    assert_eq!(m.v, [0; SIZE_REGISTERS]);
}
#[test]
fn memory_new_initializes_other() {
    let m = Memory::new();
    assert_eq!(m.pc, PROGRAM_START);
    assert_eq!(m.dt, 0);
    assert_eq!(m.st, 0);
}

#[test]
fn memory_clear_resets() {
    let empty = Memory::new();
    let mut modified = Memory::new();
    modified.ram[PROGRAM_START as usize] = 0xFF;
    modified.vram[10..1000].iter_mut().for_each(|e| *e = true);
    modified.stack.push(0xFF);
    modified.pc += 2;
    modified.dt = 10;
    modified.st = 20;
    modified.i = 1;
    modified.v.iter_mut().for_each(|e| *e = 5);
    modified.clear();
    assert_eq!(modified, empty);
}

