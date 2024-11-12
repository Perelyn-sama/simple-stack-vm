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

    fn mul(&mut self) -> Result<(), VMError> {
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(a * b);

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

    fn dup(&mut self) -> Result<(), VMError> {
        if self.stack.is_empty() {
            return Err(VMError::StackIsEmpty);
        }

        let value = self.stack[self.stack.len() - 1];

        self.push(value);

        Ok(())
    }

    fn swap(&mut self) -> Result<(), VMError> {
        if self.stack.len() < 2 {
            return Err(VMError::NotEnoughItemsInStack);
        }

        let nth = self.pop()?;
        let nth_1 = self.pop()?;

        self.push(nth);
        self.push(nth_1);

        Ok(())
    }

    fn rot(&mut self) -> Result<(), VMError> {
        if self.stack.len() < 3 {
            return Err(VMError::NotEnoughItemsInStack);
        }

        let c = self.pop()?;
        let b = self.pop()?;
        let a = self.pop()?;

        self.push(b);
        self.push(c);
        self.push(a);

        Ok(())
    }

    fn over(&mut self) -> Result<(), VMError> {
        if self.stack.len() < 2 {
            return Err(VMError::NotEnoughItemsInStack);
        }

        let value = self.stack[self.stack.len() - 2];

        self.push(value);

        Ok(())
    }

    fn pick(&mut self) -> Result<(), VMError> {
        let n = self.pop()? as usize;

        if self.stack.len() <= n {
            return Err(VMError::NotEnoughItemsInStack);
        }

        let value = self.stack[n];
        self.push(value);

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

                Opcode::Dup => {
                    self.dup()?;
                }

                Opcode::Swap => {
                    self.swap()?;
                }

                Opcode::Rot => {
                    self.rot()?;
                }

                Opcode::Over => {
                    self.over()?;
                }

                Opcode::Pick => {
                    self.pick()?;
                }

                Opcode::Mul => {
                    self.mul()?;
                }
            }

            self.pc += 1;
        }

        Ok(self.stack.clone())
    }
}

#[test]
fn test_dup_with_invalid_stack_len() {
    let mut vm = StackVM::new();

    let res = vm.dup();

    assert_eq!(Err(VMError::StackIsEmpty), res);
}

#[test]
fn test_dup() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);
    vm.push(3);

    let res = vm.dup();
    let expected_res = vec![1, 2, 3, 3];

    assert!(res.is_ok());
    assert_eq!(vm.stack, expected_res);
}

#[test]
fn test_swap_with_invalid_stack_len() {
    let mut vm = StackVM::new();

    vm.push(1);

    let res = vm.swap();

    assert_eq!(Err(VMError::NotEnoughItemsInStack), res);
}

#[test]
fn test_swap() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);
    vm.push(3);

    let res = vm.swap();
    let expected_res = vec![1, 3, 2];

    assert!(res.is_ok());
    assert_eq!(vm.stack, expected_res);
}

#[test]
fn test_rot_with_invalid_stack_len() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);

    let res = vm.rot();

    assert_eq!(Err(VMError::NotEnoughItemsInStack), res);
}

#[test]
fn test_rot() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);
    vm.push(3);

    let res = vm.rot();
    let expected_res = vec![2, 3, 1];

    assert!(res.is_ok());
    assert_eq!(vm.stack, expected_res);
}

#[test]
fn test_over_with_invalid_stack_len() {
    let mut vm = StackVM::new();

    vm.push(1);

    let res = vm.over();

    assert_eq!(Err(VMError::NotEnoughItemsInStack), res);
}

#[test]
fn test_over() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);
    vm.push(3);

    let res = vm.over();
    let expected_res = vec![1, 2, 3, 2];

    assert!(res.is_ok());
    assert_eq!(vm.stack, expected_res);
}

#[test]
fn test_pick_with_invalid_stack_len() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);

    let res = vm.pick();

    assert_eq!(Err(VMError::NotEnoughItemsInStack), res);
}

#[test]
fn test_pick() {
    let mut vm = StackVM::new();

    vm.push(1);
    vm.push(2);
    vm.push(3);
    vm.push(4);
    vm.push(2);

    let res = vm.pick();
    let expcted_res = vec![1, 2, 3, 4, 3];

    assert!(res.is_ok());
    assert_eq!(vm.stack, expcted_res);
}

#[test]
fn test_execute() {
    let mut vm = StackVM::new();

    let bytecode = vec![
        Opcode::Push as u8,
        5, // Push 5 [5]
        Opcode::Push as u8,
        3,                 // Push 3 [5, 3]
        Opcode::Add as u8, // Add them [8]
        Opcode::Push as u8,
        2,                 // Push 2 [8, 2]
        Opcode::Sub as u8, // Subtract [6]
        Opcode::Push as u8,
        0,                   // Push memory address 0 [6, 0]
        Opcode::Store as u8, // Store result []
    ];

    vm.execute(&bytecode).unwrap();
    assert_eq!(vm.memory[0], 6);
}

#[test]
fn test_execute_1() {
    let mut vm = StackVM::new();

    let bytecode = vec![
        Opcode::Push as u8,
        3,                 // Push 3 [3]
        Opcode::Dup as u8, // Duplicate 3 [3, 3]
        Opcode::Push as u8,
        4,                 // Push 4 [3, 3, 4]
        Opcode::Mul as u8, // Multiply [3, 12]
        Opcode::Push as u8,
        5,                 // Push 5 [3, 12, 5]
        Opcode::Rot as u8, // Rotate [12, 5, 3]
        Opcode::Mul as u8, // Multiply [12, 15]
        Opcode::Add as u8, // Add them [27]
        Opcode::Push as u8,
        0,                   // Push memory address 0 [27, 0]
        Opcode::Store as u8, // Store result []
    ];

    vm.execute(&bytecode).unwrap();
    assert_eq!(vm.memory[0], 27);
}

#[test]
fn test_execute_2() {
    let mut vm = StackVM::new();

    let bytecode2 = vec![
        Opcode::Push as u8,
        1, // Push 1 [1]
        Opcode::Push as u8,
        2,                  // Push 2 [1, 2]
        Opcode::Over as u8, // Copy second value [1, 2, 1]
        Opcode::Push as u8,
        3, // Push 3 [1, 2, 1, 3]
        Opcode::Push as u8,
        2,                  // Push 2 for pick [1, 2, 1, 3, 2]
        Opcode::Pick as u8, // Pick 2nd value from top (1) [1, 2, 1, 3, 1]
        Opcode::Add as u8,  // Add top two values [1, 2, 1, 4]
        Opcode::Swap as u8, // Swap top two values [1, 2, 4, 1]
    ];

    vm.execute(&bytecode2).unwrap();
    assert_eq!(vm.stack, vec![1, 2, 4, 1]);
}
