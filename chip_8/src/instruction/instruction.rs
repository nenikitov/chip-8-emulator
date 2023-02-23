#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /** Execute machine code routine. */
    System { address: u16 },
    DisplayClear,
    Jump { address: u16 },
    LoadVxValue { vx: usize, value: u16 },
    AddVxValue { vx: usize, value: u16 },
    LoadIValue { value: u16 },
    DisplayDraw { vx: usize, vy: usize, height: u16 },
}

