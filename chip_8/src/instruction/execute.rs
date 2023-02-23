use crate::memory::*;
use super::*;

/// Convert XY coordinates to the index in VRAM.
///
/// # Arguments
///
/// * `x` - X coordinate.
/// * `y` - Y coordinate.
fn coords_to_i(x: usize, y: usize) -> usize {
    x + (y * SIZE_DISPLAY.0 as usize)
}

/// Instruction that can be executes on memory.
pub trait ExecuteOnMemory {
    fn execute(&self, memory: &mut Memory);
}

impl ExecuteOnMemory for Instruction {
    fn execute(&self, memory: &mut Memory) {
        match *self {
            Instruction::System { address: _ } => {
                unimplemented!("Executing machine code is not supported")
            },
            Instruction::DisplayClear => {
                memory.vram.iter_mut().for_each(|e| *e = false)
            },
            Instruction::Jump { address } => {
                memory.pc = address;
            },
            Instruction::LoadVxValue { vx, value } => {
                memory.v[vx] = value;
            },
            Instruction::AddVxValue { vx, value } => {
                memory.v[vx] = memory.v[vx].wrapping_add(value);
            },
            Instruction::LoadIValue { value } => {
                memory.i = value;
            },
            Instruction::DisplayDraw { vx, vy, height } => {
                let x = memory.v[vx] % SIZE_DISPLAY.0;
                let y = memory.v[vy] % SIZE_DISPLAY.1;
                memory.v[0xF] = 0;
                'rows: for r in 0..(height) {
                    let row = memory.ram[(memory.i + r) as usize];
                    'pixels: for p in 0..8 {
                        let pixel = row & (1 << (7 - p));
                        let pixel = pixel != 0;
                        if pixel {
                            let x = x + p;
                            let y = y + r;
                            if x >= SIZE_DISPLAY.0 { break 'pixels; }
                            if y >= SIZE_DISPLAY.1 { break 'rows; }
                            let pos = coords_to_i(x as usize, y as usize);
                            memory.vram[pos] ^= pixel;
                            if !memory.vram[pos] {
                                memory.v[0xF] = 1;
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
fn instruction_execute_system_panics() {
    let mut m = Memory::new();
    Instruction::System { address: 0x123 }.execute(&mut m);
}

#[test]
fn instruction_execute_display_clear() {
    let mut m = Memory::new();
    m.vram.iter_mut().for_each(|e| *e = true);

    Instruction::DisplayClear.execute(&mut m);

    assert_eq!(m.vram, [false; SIZE_DISPLAY_TOTAL]);
}

#[test]
fn instruction_execute_jump() {
    let mut m = Memory::new();

    Instruction::Jump { address: 0x123 }.execute(&mut m);

    assert_eq!(m.pc, 0x123);
}

#[test]
fn instruction_execute_load_vx_value() {
    let mut m = Memory::new();

    Instruction::LoadVxValue { vx: 5, value: 0x32 }.execute(&mut m);

    assert_eq!(m.v[5], 0x32);
}

#[test]
fn instruction_execute_add_vx_value() {
    let mut m = Memory::new();
    m.v[4] = 1;

    Instruction::AddVxValue { vx: 4, value: 0x33 }.execute(&mut m);

    assert_eq!(m.v[4], 0x34);
}

#[test]
fn instruction_execute_load_i_value() {
    let mut m = Memory::new();

    Instruction::LoadIValue { value: 0x123 }.execute(&mut m);

    assert_eq!(m.i, 0x123);
}

#[test]
fn instruction_execute_display_draw() {
    let mut m = Memory::new();
    m.i = 0;
    m.ram[0] = 0b10111111;
    m.ram[1] = 0b01001001;
    m.v[4] = 1;
    m.v[6] = 2;
    m.vram[coords_to_i(1, 2)] = true;
    m.vram[coords_to_i(2, 2)] = true;
    m.vram[coords_to_i(3, 2)] = true;
    m.vram[coords_to_i(1, 3)] = true;
    m.vram[coords_to_i(2, 3)] = true;
    m.vram[coords_to_i(3, 3)] = true;

    Instruction::DisplayDraw { vx: 4, vy: 6, height: 2 }.execute(&mut m);

    assert_eq!(m.vram[coords_to_i(1, 2)], false);
    assert_eq!(m.vram[coords_to_i(2, 2)], true);
    assert_eq!(m.vram[coords_to_i(3, 2)], false);
    assert_eq!(m.vram[coords_to_i(4, 2)], true);
    assert_eq!(m.vram[coords_to_i(5, 2)], true);
    assert_eq!(m.vram[coords_to_i(6, 2)], true);
    assert_eq!(m.vram[coords_to_i(7, 2)], true);
    assert_eq!(m.vram[coords_to_i(8, 2)], true);
    assert_eq!(m.vram[coords_to_i(1, 3)], true);
    assert_eq!(m.vram[coords_to_i(2, 3)], false);
    assert_eq!(m.vram[coords_to_i(3, 3)], true);
    assert_eq!(m.vram[coords_to_i(4, 3)], false);
    assert_eq!(m.vram[coords_to_i(5, 3)], true);
    assert_eq!(m.vram[coords_to_i(6, 3)], false);
    assert_eq!(m.vram[coords_to_i(7, 3)], false);
    assert_eq!(m.vram[coords_to_i(8, 3)], true);
    assert_eq!(m.v[0xF], 1);
}

