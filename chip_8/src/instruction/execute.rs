use crate::memory::*;
use super::*;

fn coords_to_i(x: usize, y: usize) -> usize {
    x + (y * SIZE_DISPLAY.0 as usize)
}

pub trait InstructionExecutable {
    fn execute(&self, memory: &mut Memory);
}

impl InstructionExecutable for Instruction {
    fn execute(&self, memory: &mut Memory) {
        match *self {
            Instruction::CallMachineCode { address: _ } => {
                unimplemented!("Executing machine code is not supported")
            },
            Instruction::DisplayClear => {
                memory.display.iter_mut().for_each(|e| *e = false)
            },
            Instruction::FlowJump { address } => {
                memory.program_counter = address;
            },
            Instruction::RegisterSet { register, value } => {
                memory.registers_general[register] = value;
            },
            Instruction::RegisterAdd { register, value } => {
                memory.registers_general[register] = memory.registers_general[register].wrapping_add(value);
            },
            Instruction::IndexSet { value } => {
                memory.register_index = value;
            },
            Instruction::DisplayDraw { register_x, register_y, height } => {
                let x = memory.registers_general[register_x] % SIZE_DISPLAY.0;
                let y = memory.registers_general[register_y] % SIZE_DISPLAY.1;
                memory.registers_general[0xF] = 0;
                'rows: for r in 0..(height) {
                    let row = memory.ram[(memory.register_index + r) as usize];
                    'pixels: for p in 0..8 {
                        let pixel = row & (1 << (7 - p));
                        let pixel = pixel != 0;
                        if pixel {
                            let x = x + p;
                            let y = y + r;
                            if x >= SIZE_DISPLAY.0 { break 'pixels; }
                            if y >= SIZE_DISPLAY.1 { break 'rows; }
                            let pos = coords_to_i(x as usize, y as usize);
                            memory.display[pos] ^= pixel;
                            if !memory.display[pos] {
                                memory.registers_general[0xF] = 1;
                            }
                        }
                    }
                }
            }
        };
    }
}


#[test]
#[should_panic(expected = "not implemented: Executing machine code is not supported")]
fn instruction_executes_on_memory_machine_code_panics() {
    let mut m = Memory::new();
    Instruction::CallMachineCode { address: 0x123 }.execute(&mut m);
}

#[test]
fn instruction_executes_on_memory_display_clear() {
    let mut m = Memory::new();
    m.display.iter_mut().for_each(|e| *e = true);

    Instruction::DisplayClear.execute(&mut m);

    assert_eq!(m.display, [false; SIZE_DISPLAY_TOTAL]);
}

#[test]
fn instruction_executes_on_memory_flow_jump() {
    let mut m = Memory::new();

    Instruction::FlowJump { address: 0x123 }.execute(&mut m);

    assert_eq!(m.program_counter, 0x123);
}

#[test]
fn instruction_executes_on_memory_register_set() {
    let mut m = Memory::new();

    Instruction::RegisterSet { register: 5, value: 0x32 }.execute(&mut m);

    assert_eq!(m.registers_general[5], 0x32);
}

#[test]
fn instruction_executes_on_memory_register_add() {
    let mut m = Memory::new();
    m.registers_general[4] = 1;

    Instruction::RegisterAdd { register: 4, value: 0x33 }.execute(&mut m);

    assert_eq!(m.registers_general[4], 0x34);
}

#[test]
fn instruction_executes_on_memory_index_set() {
    let mut m = Memory::new();

    Instruction::IndexSet { value: 0x123 }.execute(&mut m);

    assert_eq!(m.register_index, 0x123);
}

#[test]
fn instruction_executes_on_memory_display_draw() {
    let mut m = Memory::new();
    m.register_index = 0;
    m.ram[0] = 0b10111111;
    m.ram[1] = 0b01001001;
    m.registers_general[4] = 1;
    m.registers_general[6] = 2;
    m.display[coords_to_i(1, 2)] = true;
    m.display[coords_to_i(2, 2)] = true;
    m.display[coords_to_i(3, 2)] = true;
    m.display[coords_to_i(1, 3)] = true;
    m.display[coords_to_i(2, 3)] = true;
    m.display[coords_to_i(3, 3)] = true;

    Instruction::DisplayDraw { register_x: 4, register_y: 6, height: 2 }.execute(&mut m);

    assert_eq!(m.display[coords_to_i(1, 2)], false);
    assert_eq!(m.display[coords_to_i(2, 2)], true);
    assert_eq!(m.display[coords_to_i(3, 2)], false);
    assert_eq!(m.display[coords_to_i(4, 2)], true);
    assert_eq!(m.display[coords_to_i(5, 2)], true);
    assert_eq!(m.display[coords_to_i(6, 2)], true);
    assert_eq!(m.display[coords_to_i(7, 2)], true);
    assert_eq!(m.display[coords_to_i(8, 2)], true);
    assert_eq!(m.display[coords_to_i(1, 3)], true);
    assert_eq!(m.display[coords_to_i(2, 3)], false);
    assert_eq!(m.display[coords_to_i(3, 3)], true);
    assert_eq!(m.display[coords_to_i(4, 3)], false);
    assert_eq!(m.display[coords_to_i(5, 3)], true);
    assert_eq!(m.display[coords_to_i(6, 3)], false);
    assert_eq!(m.display[coords_to_i(7, 3)], false);
    assert_eq!(m.display[coords_to_i(8, 3)], true);
    assert_eq!(m.registers_general[0xF], 1);
}

