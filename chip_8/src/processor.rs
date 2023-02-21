use crate::{instructions::*, memory::*};

#[derive(Debug)]
pub struct Processor {
    pub memory: Memory,
}

struct Opcode {
    i: usize,
    x: usize,
    y: usize,
    n: usize,
    nn: u16,
    nnn: u16,
}
impl Opcode {
    pub fn to_tuple(&self) -> (usize, usize, usize, usize, u16, u16) {
        (
            self.i,
            self.x,
            self.y,
            self.n,
            self.nn,
            self.nnn
        )
    }
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
        let opcode = self.read_instruction_bytes();
        let opcode = Self::split_opcode(opcode);
        let (i, x, y, n, nn, nnn) = opcode.to_tuple();
        match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
            (0x1, _,   _,   _  ) => Instruction::Jump { address: nnn },
            (0x6, _,   _,   _  ) => Instruction::RegisterSet { register: x, value: nn },
            (0x7, _,   _,   _  ) => Instruction::RegisterAdd { register: x, value: nn },
            (0xA, _,   _,   _  ) => Instruction::IndexSet { value: nnn },
            (0xD, _,   _,   _  ) => Instruction::Display { register_x: x, register_y: y, height: n },
            _ => unreachable!()
        }
    }

    fn read_instruction_bytes(&mut self) -> u16 {
        let (byte_1, byte_2) = (
            self.memory.ram[self.memory.program_couter as usize],
            self.memory.ram[self.memory.program_couter as usize + 1],
        );
        self.memory.program_couter += 1;

        ((byte_1 as u16) << 8) | (byte_2 as u16)
    }

    fn split_opcode(bytes: u16) -> Opcode {
        let i = ((bytes & 0xF000) >> 12) as usize;
        let x = ((bytes & 0x0F00) >> 8) as usize;
        let y = ((bytes & 0x00F0) >> 4) as usize;
        let n = (bytes & 0x000F) as usize;
        let nn = (bytes & 0x00FF) as u16;
        let nnn = (bytes & 0x0FFF) as u16;

        Opcode { i, x, y, n, nn, nnn }
    }
}


#[test]
fn split_opcode_splits() {
    let opcode = Processor::split_opcode(0xD123);
    assert_eq!(opcode.i, 0xD);
    assert_eq!(opcode.x, 0x1);
    assert_eq!(opcode.y, 0x2);
    assert_eq!(opcode.n, 0x3);
    assert_eq!(opcode.nn, 0x23);
    assert_eq!(opcode.nnn, 0x123);
}

#[test]
fn split_opcode_converts_to_tuple() {
    let opcode = Processor::split_opcode(0xD123);
    let (i, x, y, n, nn, nnn) = opcode.to_tuple();
    assert_eq!(i, 0xD);
    assert_eq!(x, 0x1);
    assert_eq!(y, 0x2);
    assert_eq!(n, 0x3);
    assert_eq!(nn, 0x23);
    assert_eq!(nnn, 0x123);
}

#[test]
fn processor_fetches_the_instructions_correctly() {
}

