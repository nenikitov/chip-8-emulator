use super::*;

impl From<Opcode> for Instruction {
    fn from(o: Opcode) -> Self {
        let (i, x, y, n, nn, nnn) = o.into();
        match (i, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Instruction::DisplayClear,
            (0x0, _,   _,   _  ) => Instruction::CallMachineCode { address: nnn },
            (0x1, _,   _,   _  ) => Instruction::FlowJump { address: nnn },
            (0x6, _,   _,   _  ) => Instruction::RegisterSet { register: x, value: nn },
            (0x7, _,   _,   _  ) => Instruction::RegisterAdd { register: x, value: nn },
            (0xA, _,   _,   _  ) => Instruction::IndexSet { value: nnn },
            (0xD, _,   _,   _  ) => Instruction::DisplayDraw { register_x: x, register_y: y, height: n as u16 },
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
fn instruction_from_opcode_0nnn_returns_call_machine_code() {
    assert_eq!(
        Instruction::from(Opcode::from(0x0123)),
        Instruction::CallMachineCode { address: 0x123 }
    )
}

#[test]
fn instruction_from_opcode_1nnn_returns_flow_jump() {
    assert_eq!(
        Instruction::from(Opcode::from(0x1123)),
        Instruction::FlowJump { address: 0x123 }
    )
}

#[test]
fn instruction_from_opcode_6xnn_returns_register_set() {
    assert_eq!(
        Instruction::from(Opcode::from(0x6123)),
        Instruction::RegisterSet { register: 0x1, value: 0x23 }
    )
}

#[test]
fn instruction_from_opcode_7xnn_returns_register_add() {
    assert_eq!(
        Instruction::from(Opcode::from(0x7123)),
        Instruction::RegisterAdd { register: 0x1, value: 0x23 }
    )
}

#[test]
fn instruction_from_opcode_axnn_returns_index_set() {
    assert_eq!(
        Instruction::from(Opcode::from(0xA123)),
        Instruction::IndexSet { value: 0x123 }
    )
}

#[test]
fn instruction_from_opcode_dxyn_returns_display_draw() {
    assert_eq!(
        Instruction::from(Opcode::from(0xD123)),
        Instruction::DisplayDraw { register_x: 0x1, register_y: 0x2, height: 0x3 }
    )
}

