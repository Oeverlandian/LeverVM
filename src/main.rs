use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    
    // Arithmetic 
    ADD,     // Add two values
    SUB,     // Subtract two values
    MUL,     // Multiply two values
    DIV,     // Divide two values
    MOD,     // Find the remainder of two values
    INC,     // Increment the value by one
    DEC,     // Decrement the value by one

    // Data
    PUSH,    // Push value onto stack
    POP,     // Pop value from stack
    STORE,   // Store value in memory
    LOAD,    // Load value from memory

    // IO
    IN,     // Gets input from the console and pushes it on to the stack
    PRINT,  // Print the last thing on the stack to the console

    // Miscellaneous 
    HALT,   // Halt execution
    NOOP,   // No operation
}

pub struct VM {
    stack: Vec<i32>,
    memory: HashMap<usize, i32>,
    program: Vec<(Opcode, Option<i32>)>,
    pc: usize,  // Program counter
    running: bool,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            memory: HashMap::new(),
            program: Vec::new(),
            pc: 0,
            running: false,
        }
    }

    pub fn load_program(&mut self, program: Vec<(Opcode, Option<i32>)>) {
        self.program = program;
        self.pc = 0;
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running && self.pc < self.program.len() {
            self.execute_instruction();
            self.pc += 1;
        }
    }

    fn execute_instruction(&mut self) {
        let (opcode, operand) = self.program[self.pc];
        
        match opcode {
            Opcode::ADD => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(a + b);
                }
            },
            Opcode::SUB => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(a - b);
                }
            },
            Opcode::MUL => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(a * b);
                }
            },
            Opcode::DIV => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    if b != 0 {
                        self.stack.push(a / b);
                    }
                }
            },
            Opcode::MOD => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(a % b);
                }
            }
            Opcode::INC => {
                if let Some(a) = self.stack.pop() {
                    self.stack.push(a + 1);
                }
            }
            Opcode::DEC => {
                if let Some(a) = self.stack.pop() {
                    self.stack.push(a - 1);
                }
            }
            Opcode::PUSH => {
                if let Some(value) = operand {
                    self.stack.push(value);
                }
            },
            Opcode::POP => {
                self.stack.pop();
            },
            Opcode::STORE => {
                if let (Some(value), Some(address)) = (self.stack.pop(), operand) {
                    self.memory.insert(address as usize, value);
                }
            },
            Opcode::LOAD => {
                if let Some(address) = operand {
                    if let Some(&value) = self.memory.get(&(address as usize)) {
                        self.stack.push(value);
                    }
                }
            },
            Opcode::IN => {
                let mut input_line = String::new();
                std::io::stdin()
                    .read_line(&mut input_line)
                    .expect("Failed to read line");
                let a: i32 = input_line.trim().parse().expect("Input not an integer");
                self.stack.push(a);
            }
            Opcode::PRINT => {
                if let Some(value) = Some(self.stack.pop()) {
                    let print_value = value.unwrap();
                    println!("{}", print_value);
                } else {
                    eprintln!("The stack is empty")
                }
            },
            Opcode::HALT => {
                self.running = false;
            },
            Opcode::NOOP => {
                // Does nothing
            },
        }
    }
}

fn main() {
    let mut vm = VM::new();

    let program = vec![
        (Opcode::PUSH, Some(5)),    // Push 5
        (Opcode::IN, None),         // Push user input
        (Opcode::ADD, None),        // Add them
        (Opcode::PRINT, None),      // Prints result to the console
        (Opcode::HALT, None),       // Stop execution
    ];

    vm.load_program(program);
    vm.run();
}