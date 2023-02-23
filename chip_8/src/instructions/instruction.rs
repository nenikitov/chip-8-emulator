#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    CallMachineCode { address: u16 },
    DisplayClear,
    FlowJump { address: u16 },
    RegisterSet { register: usize, value: u16 },
    RegisterAdd { register: usize, value: u16 },
    IndexSet { value: u16 },
    DisplayDraw { register_x: usize, register_y: usize, height: u16 },
}

