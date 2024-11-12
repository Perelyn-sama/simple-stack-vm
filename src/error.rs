use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum VMError {
    StackUnderflow,
    MemoryOutOfBounds,
    InvalidJumpTarget,
    InvalidOpcode,
    InvalidProgramCounter,
    StackIsEmpty,
    NotEnoughItemsInStack,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::MemoryOutOfBounds => write!(f, "Memory access out of bounds"),
            VMError::InvalidJumpTarget => write!(f, "Jump target outside program bounds"),
            VMError::InvalidOpcode => write!(f, "Invalid opcode"),
            VMError::InvalidProgramCounter => write!(f, "Program counter exceeded bytecode length"),
            VMError::StackIsEmpty => write!(f, "Stack is empty"),
            VMError::NotEnoughItemsInStack => write!(f, "Not enough items in stack"),
        }
    }
}

impl Error for VMError {}
