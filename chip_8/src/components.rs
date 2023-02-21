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


#[derive(Debug)]
pub struct Components {
    pub ram: [u8; SIZE_RAM],
    pub display: [bool; SIZE_DISPLAY],
    pub stack: Vec<u16>,
    pub program_couter: u16,
    pub timer_delay: u8,
    pub timer_sound: u8,
    pub regiser_index: u16,
    pub registers_general: [u16; 16]
}

impl Components {
    pub fn new() -> Self {
        let mut ram = [0; SIZE_RAM];
        ram[0x50..0xA0].copy_from_slice(&FONT);
        Self {
            ram,
            display: [false; SIZE_DISPLAY],
            stack: Vec::new(),
            program_couter: 0,
            timer_delay: 0,
            timer_sound: 0,
            regiser_index: 0,
            registers_general: [0; SIZE_REGISTERS]
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn components_new_initializes_ram() {
        let c = Components::new();
        assert_eq!(c.ram[0..0x50], [0; 0x50]);
        assert_eq!(c.ram[0x50..0xA0], FONT);
        assert_eq!(c.ram[0xA0..SIZE_RAM], [0; SIZE_RAM - 0xA0]);
    }

    #[test]
    fn components_new_initializes_display() {
        let c = Components::new();
        assert_eq!(c.display, [false; SIZE_DISPLAY]);
    }

    #[test]
    fn components_new_initializes_stack() {
        let c = Components::new();
        assert_eq!(c.stack, [0; 0]);
    }

    #[test]
    fn components_new_initializes_regitsers() {
        let c = Components::new();
        assert_eq!(c.regiser_index, 0);
        assert_eq!(c.registers_general, [0; SIZE_REGISTERS]);
    }

    #[test]
    fn components_new_initializes_other() {
        let c = Components::new();
        assert_eq!(c.program_couter, 0);
        assert_eq!(c.timer_delay, 0);
        assert_eq!(c.timer_sound, 0);
    }
}

