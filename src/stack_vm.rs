use crate::error::*;
use crate::opcode::*;

pub struct StackVM {
    pub stack: Vec<i32>,
    pub memory: [i32; 256],
    pub pc: usize,
}

impl StackVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            memory: [0; 256],
            pc: 0,
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<i32, VMError> {
        self.stack.pop().ok_or(VMError::StackUnderflow)
    }

    fn add(&mut self) -> Result<(), VMError> {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(a + b);

        Ok(())
    }

    fn sub(&mut self) -> Result<(), VMError> {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(a - b);

        Ok(())
    }

    fn store(&mut self) -> Result<(), VMError> {
        let addr = self.pop()? as usize;
        let value = self.pop()?;

        if addr >= self.memory.len() {
            return Err(VMError::MemoryOutOfBounds);
        }

        self.memory[addr] = value;

        Ok(())
    }

    fn load(&mut self) -> Result<(), VMError> {
        let addr = self.pop()? as usize;
        if addr >= self.memory.len() {
            return Err(VMError::MemoryOutOfBounds);
        }

        self.push(self.memory[addr]);

        Ok(())
    }

    fn jump(&mut self, bytecode_len: usize) -> Result<(), VMError> {
        let target = self.pop()? as usize;
        if target >= bytecode_len {
            return Err(VMError::InvalidJumpTarget);
        }

        self.pc = target;

        Ok(())
    }

    fn jump_if(&mut self, bytecode_len: usize) -> Result<(), VMError> {
        let cond = self.pop()?;
        let target = self.pop()? as usize;

        if target >= bytecode_len {
            return Err(VMError::InvalidJumpTarget);
        }

        if cond != 0 {
            self.pc = target;
        }

        Ok(())
    }

    pub fn execute(&mut self, bytecode: &[u8]) -> Result<Vec<i32>, VMError> {
        while self.pc < bytecode.len() {
            let opcode = Opcode::try_from(bytecode[self.pc])?;

            match opcode {
                Opcode::Push => {
                    self.pc += 1;

                    if self.pc >= bytecode.len() {
                        return Err(VMError::InvalidProgramCounter);
                    }

                    self.push(bytecode[self.pc] as i32);
                }

                Opcode::Pop => {
                    self.pop()?;
                }

                Opcode::Add => {
                    self.add()?;
                }

                Opcode::Sub => {
                    self.sub()?;
                }

                Opcode::Store => {
                    self.store()?;
                }

                Opcode::Load => {
                    self.load()?;
                }

                Opcode::Jmp => {
                    self.jump(bytecode.len())?;
                    continue;
                }

                Opcode::JumpIf => {
                    self.jump_if(bytecode.len())?;
                    continue;
                }
            }

            self.pc += 1;
        }

        Ok(self.stack.clone())
    }
}
