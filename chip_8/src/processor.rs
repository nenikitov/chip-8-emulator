use crate::{instructions::*, memory::*};

#[derive(Debug)]
pub struct Processor {
    pub memory: Memory,
}

struct SplitBytes {
    i: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
}

impl Processor {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        todo!()
    }

    pub fn fetch(&mut self) -> Instruction {
        let instruction_bytes = self.read_instruction_bytes();
        let instruction_bytes = Self::split_instruction_bytes(instruction_bytes);
        todo!()
    }

    fn read_instruction_bytes(&mut self) -> u16 {
        let (byte_1, byte_2) = (
            self.memory.ram[self.memory.program_couter as usize],
            self.memory.ram[self.memory.program_couter as usize + 1],
        );
        self.memory.program_couter += 1;

        ((byte_1 as u16) << 8) | (byte_2 as u16)
    }

    fn split_instruction_bytes(bytes: u16) -> SplitBytes {
        let i = ((bytes & 0xF000) >> 12) as u8;
        let x = ((bytes & 0x0F00) >> 8) as u8;
        let y = ((bytes & 0x00F0) >> 4) as u8;
        let n = (bytes & 0x000F) as u8;
        let nn = (bytes & 0x00FF) as u8;
        let nnn = bytes & 0x0FFF;

        SplitBytes { i, x, y, n, nn, nnn }
    }
}


#[test]
fn split_instruction_bytes_splits_correctly() {
    let bytes = Processor::split_instruction_bytes(0xD123);
    assert_eq!(bytes.i, 0xD);
    assert_eq!(bytes.x, 0x1);
    assert_eq!(bytes.y, 0x2);
    assert_eq!(bytes.n, 0x3);
    assert_eq!(bytes.nn, 0x23);
    assert_eq!(bytes.nnn, 0x123);
}

