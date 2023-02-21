#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump { address: u16 },
    RegisterSet { register: usize, value: u16 },
    RegisterAdd { register: usize, value: u16 },
    IndexSet { value: u16 },
    Display { register_x: usize, register_y: usize, height: usize },
}

