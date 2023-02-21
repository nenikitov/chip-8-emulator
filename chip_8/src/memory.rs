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
const SIZE_DISPLAY: usize = 64 * 32;
const SIZE_REGISTERS: usize = 16;
pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Memory {
    pub ram: [u8; SIZE_RAM],
    pub display: [bool; SIZE_DISPLAY],
    pub stack: Vec<u16>,
    pub program_couter: u16,
    pub timer_delay: u8,
    pub timer_sound: u8,
    pub regiser_index: u16,
    pub registers_general: [u16; 16]
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self {
            ram: [0; SIZE_RAM],
            display: [false; SIZE_DISPLAY],
            stack: Vec::new(),
            program_couter: PROGRAM_START,
            timer_delay: 0,
            timer_sound: 0,
            regiser_index: 0,
            registers_general: [0; SIZE_REGISTERS]
        };
        memory.clear();
        memory
    }

    pub fn clear(&mut self) {
        self.ram.iter_mut().for_each(|e| *e = 0);
        self.ram[0x50..0xA0].copy_from_slice(&FONT);
        self.display.iter_mut().for_each(|e| *e = false);
        self.stack.clear();
        self.registers_general.iter_mut().for_each(|e| *e = 0);
        self.program_couter = PROGRAM_START;
        self.timer_delay = 0;
        self.timer_sound = 0;
        self.regiser_index = 0;
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
    assert_eq!(m.display, [false; SIZE_DISPLAY]);
}
#[test]
fn memory_new_initializes_stack() {
    let m = Memory::new();
    assert_eq!(m.stack, [0; 0]);
}
#[test]
fn memory_new_initializes_regitsers() {
    let m = Memory::new();
    assert_eq!(m.regiser_index, 0);
    assert_eq!(m.registers_general, [0; SIZE_REGISTERS]);
}
#[test]
fn memory_new_initializes_other() {
    let m = Memory::new();
    assert_eq!(m.program_couter, PROGRAM_START);
    assert_eq!(m.timer_delay, 0);
    assert_eq!(m.timer_sound, 0);
}

#[test]
fn memory_clear_resets() {
    let empty = Memory::new();
    let mut modified = Memory::new();
    modified.ram[PROGRAM_START as usize] = 0xFF;
    modified.display[10..1000].iter_mut().for_each(|e| *e = true);
    modified.stack.push(0xFF);
    modified.program_couter += 2;
    modified.timer_delay = 10;
    modified.timer_sound = 20;
    modified.regiser_index = 1;
    modified.registers_general.iter_mut().for_each(|e| *e = 5);
    modified.clear();
    assert_eq!(modified, empty);
}

