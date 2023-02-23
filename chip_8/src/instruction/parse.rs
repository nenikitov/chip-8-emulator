use super::*;

impl From<Opcode> for Instruction {
    fn from(o: Opcode) -> Self {
        let (i, x, y, n, nn, nnn) = o.into();
        match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::DisplayClear,
            (0x0, _,   _,   _  ) => Instruction::System { address: nnn },
            (0x1, _,   _,   _  ) => Instruction::Jump { address: nnn },
            (0x6, _,   _,   _  ) => Instruction::LoadVxValue { vx: x, value: nn },
            (0x7, _,   _,   _  ) => Instruction::AddVxValue { vx: x, value: nn },
            (0xA, _,   _,   _  ) => Instruction::LoadIValue { value: nnn },
            (0xD, _,   _,   _  ) => Instruction::DisplayDraw { vx: x, vy: y, height: n as u16 },
            _ => unreachable!("Unknown instruction {:X}{:X}{:X}{:X}", i, x, y, n)
        }
    }
}

#[test]
fn instruction_from_opcode_00e0_returns_display_clear() {
    assert_eq!(
        Instruction::from(Opcode::from(0x00E0)),
        Instruction::DisplayClear
    )
}

#[test]
fn instruction_from_opcode_0nnn_returns_system() {
    assert_eq!(
        Instruction::from(Opcode::from(0x0123)),
        Instruction::System { address: 0x123 }
    )
}

#[test]
fn instruction_from_opcode_1nnn_returns_jump() {
    assert_eq!(
        Instruction::from(Opcode::from(0x1123)),
        Instruction::Jump { address: 0x123 }
    )
}

#[test]
fn instruction_from_opcode_6xnn_returns_load_vx_value() {
    assert_eq!(
        Instruction::from(Opcode::from(0x6123)),
        Instruction::LoadVxValue { vx: 0x1, value: 0x23 }
    )
}

#[test]
fn instruction_from_opcode_7xnn_returns_add_vx_value() {
    assert_eq!(
        Instruction::from(Opcode::from(0x7123)),
        Instruction::AddVxValue { vx: 0x1, value: 0x23 }
    )
}

#[test]
fn instruction_from_opcode_axnn_returns_load_i_value() {
    assert_eq!(
        Instruction::from(Opcode::from(0xA123)),
        Instruction::LoadIValue { value: 0x123 }
    )
}

#[test]
fn instruction_from_opcode_dxyn_returns_display_draw() {
    assert_eq!(
        Instruction::from(Opcode::from(0xD123)),
        Instruction::DisplayDraw { vx: 0x1, vy: 0x2, height: 0x3 }
    )
}

