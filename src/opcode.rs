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
            _ => Err(VMError::InvalidOpcode),
        }
    }
}
