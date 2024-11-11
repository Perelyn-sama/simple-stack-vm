use simple_stack_vm::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = StackVM::new();

    // Program: Calculate (5 + 3) - 2 and store in memory
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

    vm.execute(&bytecode)?;
    println!("Memory at address 0: {}", vm.memory[0]); // Should print 6

    Ok(())
}
