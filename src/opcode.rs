use crate::error::*;

#[derive(Debug)]
pub enum Opcode {
    Push = 0x01,
    Pop = 0x02,
    Add = 0x03,
    Sub = 0x04,
    Store = 0x05,
    Load = 0x06,
    Jmp = 0x07,
    JumpIf = 0x08,
    Dup = 0x09,
    Swap = 0x10,
    Rot = 0x11,
    Over = 0x12,
    Pick = 0x13,
    Mul = 0x14,
}

impl TryFrom<u8> for Opcode {
    type Error = VMError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Opcode::Push),
            0x02 => Ok(Opcode::Pop),
            0x03 => Ok(Opcode::Add),
            0x04 => Ok(Opcode::Sub),
            0x05 => Ok(Opcode::Store),
            0x06 => Ok(Opcode::Load),
            0x07 => Ok(Opcode::Jmp),
            0x08 => Ok(Opcode::JumpIf),
            0x09 => Ok(Opcode::Dup),
            0x10 => Ok(Opcode::Swap),
            0x11 => Ok(Opcode::Rot),
            0x12 => Ok(Opcode::Over),
            0x13 => Ok(Opcode::Pick),
            0x14 => Ok(Opcode::Mul),
            _ => Err(VMError::InvalidOpcode),
        }
    }
}
