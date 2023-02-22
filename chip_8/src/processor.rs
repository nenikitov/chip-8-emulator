use crate::{instructions::*, memory::*};

#[derive(Debug)]
pub struct Processor {
    pub memory: Memory,
}

pub struct Opcode {
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


impl Processor {
    pub fn new() -> Self {
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
                self.memory.program_counter = address;
            },
            Instruction::RegisterSet { register, value } => {
                self.memory.registers_general[register] = value;
            },
            Instruction::RegisterAdd { register, value } => {
                self.memory.registers_general[register] = self.memory.registers_general[register].wrapping_add(value);
            },
            Instruction::IndexSet { value } => {
                self.memory.register_index = value;
            },
            Instruction::Display { register_x, register_y, height } => {
                let x = self.memory.registers_general[register_x] % SIZE_DISPLAY.0;
                let y = self.memory.registers_general[register_y] % SIZE_DISPLAY.1;
                self.memory.registers_general[0xF] = 0;
                'rows: for r in 0..(height) {
                    let row = self.memory.ram[(self.memory.register_index + r) as usize];
                    'pixels: for p in 0..8 {
                        let pixel = row & (1 << (7 - p));
                        let pixel = pixel != 0;
                        if pixel {
                            let x = x + p;
                            let y = y + r;
                            if x >= SIZE_DISPLAY.0 { break 'pixels; }
                            if y >= SIZE_DISPLAY.1 { break 'rows; }
                            let pos = (x + y * SIZE_DISPLAY.0) as usize;
                            self.memory.display[pos] ^= pixel;
                            if !self.memory.display[pos] {
                                self.memory.registers_general[0xF] = 1;
                            }
                        }
                    }
                }
            },
        }
    }

    fn parse(opcode: Opcode) -> Instruction {
        let (i, x, y, n, nn, nnn) = opcode.to_tuple();
        match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
            (0x0, _,   _,   _  ) => Instruction::ExecuteMachineCode { address: nnn },
            (0x1, _,   _,   _  ) => Instruction::Jump { address: nnn },
            (0x6, _,   _,   _  ) => Instruction::RegisterSet { register: x, value: nn },
            (0x7, _,   _,   _  ) => Instruction::RegisterAdd { register: x, value: nn },
            (0xA, _,   _,   _  ) => Instruction::IndexSet { value: nnn },
            (0xD, _,   _,   _  ) => Instruction::Display { register_x: x, register_y: y, height: n as u16 },
            _ => unreachable!("Unknown instruction {:X}{:X}{:X}{:X}", i, x, y, n)
        }
    }

    pub fn fetch(&mut self) -> Instruction {
        let opcode = self.read_instruction_bytes();
        Self::parse(Opcode::from(opcode))
    }

    fn read_instruction_bytes(&mut self) -> u16 {
        let (byte_1, byte_2) = (
            self.memory.ram[self.memory.program_counter as usize],
            self.memory.ram[self.memory.program_counter as usize + 1],
        );
        self.memory.program_counter += 2;

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
fn processor_runs_00e0() {
    let mut p = Processor::new();
    p.memory.display.iter_mut().for_each(|e| *e = true);
    p.execute(Processor::parse(Opcode::from(0x00E0)));

    assert_eq!(p.memory.display, [false; SIZE_DISPLAY_TOTAL]);
}

#[test]
fn processor_runs_1nnn() {
    let mut p = Processor::new();
    p.execute(Processor::parse(Opcode::from(0x1666)));
    assert_eq!(p.memory.program_counter, 0x0666);
}

#[test]
fn processor_runs_6xnn() {
    let mut p = Processor::new();
    p.execute(Processor::parse(Opcode::from(0x6532)));
    assert_eq!(p.memory.registers_general[5], 0x32);
}

#[test]
fn processor_runs_7xnn() {
    let mut p = Processor::new();
    p.memory.registers_general[5] = 1;
    p.execute(Processor::parse(Opcode::from(0x7532)));
    assert_eq!(p.memory.registers_general[5], 0x33);
}

#[test]
fn processor_runs_annn() {
    let mut p = Processor::new();
    p.execute(Processor::parse(Opcode::from(0xa123)));
    assert_eq!(p.memory.register_index, 0x123);
}

#[test]
fn processor_runs_dxyn() {
    fn get_i(x: usize, y: usize) -> usize {
        x + (y * SIZE_DISPLAY.0 as usize)
    }

    let mut p = Processor::new();
    p.memory.register_index = 0;
    p.memory.ram[0] = 0b10111111;
    p.memory.ram[1] = 0b01001001;
    p.memory.registers_general[2] = 1;
    p.memory.registers_general[6] = 2;

    p.memory.display[get_i(1, 2)] = true;
    p.memory.display[get_i(2, 2)] = true;
    p.memory.display[get_i(3, 2)] = true;
    p.memory.display[get_i(1, 3)] = true;
    p.memory.display[get_i(2, 3)] = true;
    p.memory.display[get_i(3, 3)] = true;

    p.execute(Processor::parse(Opcode::from(0xD262)));

    assert_eq!(p.memory.display[get_i(1, 2)], false);
    assert_eq!(p.memory.display[get_i(2, 2)], true);
    assert_eq!(p.memory.display[get_i(3, 2)], false);
    assert_eq!(p.memory.display[get_i(4, 2)], true);
    assert_eq!(p.memory.display[get_i(5, 2)], true);
    assert_eq!(p.memory.display[get_i(6, 2)], true);
    assert_eq!(p.memory.display[get_i(7, 2)], true);
    assert_eq!(p.memory.display[get_i(8, 2)], true);
    assert_eq!(p.memory.display[get_i(1, 3)], true);
    assert_eq!(p.memory.display[get_i(2, 3)], false);
    assert_eq!(p.memory.display[get_i(3, 3)], true);
    assert_eq!(p.memory.display[get_i(4, 3)], false);
    assert_eq!(p.memory.display[get_i(5, 3)], true);
    assert_eq!(p.memory.display[get_i(6, 3)], false);
    assert_eq!(p.memory.display[get_i(7, 3)], false);
    assert_eq!(p.memory.display[get_i(8, 3)], true);

    assert_eq!(p.memory.registers_general[0xF], 1);
}

