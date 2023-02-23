#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /** Execute machine code routine. */
    System { address: u16 },
    DisplayClear,
    Jump { address: u16 },
    LoadVxValue { register: usize, value: u16 },
    AddVxValue { register: usize, value: u16 },
    LoadIValue { value: u16 },
    DisplayDraw { register_x: usize, register_y: usize, height: u16 },
}

