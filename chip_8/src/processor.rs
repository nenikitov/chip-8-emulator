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
    pub fn from(instruction: u16) -> Self {
        let i = ((instruction & 0xF000) >> 12) as usize;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        let n = (instruction & 0x000F) as usize;
        let nn = (instruction & 0x00FF) as u16;
        let nnn = (instruction & 0x0FFF) as u16;

        Self { i, x, y, n, nn, nnn }
    }

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


impl Processor { pub fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ExecuteMachineCode { address: _ } => {
                panic!("Executing machine code is not supported")
            },
            Instruction::ClearScreen => {
                self.memory.display.iter_mut().for_each(|e| *e = false)
            },
            Instruction::Jump { address } => {
                self.memory.program_couter = address;
            },
            Instruction::RegisterSet { register, value } => {
                self.memory.registers_general[register] = value;
            },
            Instruction::RegisterAdd { register, value } => {
                self.memory.registers_general[register] = self.memory.registers_general[register].wrapping_add(value);
            },
            Instruction::IndexSet { value } => {
                self.memory.regiser_index = value;
            },
            Instruction::Display { register_x, register_y, height } => {
                let x = self.memory.registers_general[register_x] % SIZE_DISPLAY.0;
                let y = self.memory.registers_general[register_y] % SIZE_DISPLAY.1;
                self.memory.registers_general[0xF] = 0;
                'rows: for i in 0..height {
                    let row = self.memory.ram[(self.memory.regiser_index + i) as usize];
                    'pixels: for j in 0..8 {
                        let pixel = row & (1 << j);
                        let pixel = pixel != 0;
                        let x = x + j;
                        let y = y + i;
                        if x >= SIZE_DISPLAY.0 { break 'pixels; }
                        if y >= SIZE_DISPLAY.1 { break 'rows; }
                        let pos = (x + y * SIZE_DISPLAY.0) as usize;
                        self.memory.display[pos] ^= pixel;
                        if !self.memory.display[pos] {
                            self.memory.registers_general[0xF] = 1;
                        }
                    }
                }
            },
        }
    }

    pub fn fetch(&mut self) -> Instruction {
        let opcode = self.read_instruction_bytes();
        let opcode = Opcode::from(opcode);
        let (i, x, y, n, nn, nnn) = opcode.to_tuple();
        match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
            (0x0, _,   _,   _  ) => Instruction::ExecuteMachineCode { address: nnn },
            (0x1, _,   _,   _  ) => Instruction::Jump { address: nnn },
            (0x6, _,   _,   _  ) => Instruction::RegisterSet { register: x, value: nn },
            (0x7, _,   _,   _  ) => Instruction::RegisterAdd { register: x, value: nn },
            (0xA, _,   _,   _  ) => Instruction::IndexSet { value: nnn },
            (0xD, _,   _,   _  ) => Instruction::Display { register_x: x, register_y: y, height: n as u16 },
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
}


#[test]
fn opcode_from_u16_splits() {
    let opcode = Opcode::from(0xD123);
    assert_eq!(opcode.i, 0xD);
    assert_eq!(opcode.x, 0x1);
    assert_eq!(opcode.y, 0x2);
    assert_eq!(opcode.n, 0x3);
    assert_eq!(opcode.nn, 0x23);
    assert_eq!(opcode.nnn, 0x123);
}

#[test]
fn opcode_to_tuple_converts() {
    let opcode = Opcode::from(0xD123);
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

