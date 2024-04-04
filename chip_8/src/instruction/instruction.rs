/// CPU instruction with required arguments.
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Execute machine code routine at address.
    System { address: u16 },
    /// Clear the display (VRAM).
    DisplayClear,
    /// Jump to an instruction at address.
    Jump { address: u16 },
    /// Load a value into register Vx.
    LoadVxValue { vx: usize, value: u16 },
    /// Add a value to register Vx.
    AddVxValue { vx: usize, value: u16 },
    /// Load a value into register I.
    LoadIValue { value: u16 },
    /// Display a sprite from register I with specified height in the coordinates from registers Vx and Vy.
    DisplayDraw { vx: usize, vy: usize, height: u16 },
}
