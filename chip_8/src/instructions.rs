#[derive(Debug)]
pub enum Instruction {
    ClearScreen,
    Jump { address: u16 },
    RegisterSet { register: usize, value: u16 },
    RegisterAdd { register: usize, value: u16 },
    IndexSet { value: u16 },
    Display { registerX: usize, registerY: usize, height: usize }
}

