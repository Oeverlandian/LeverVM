use std::collections::HashMap;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    
    // Arithmetic 
    ADD,      // Add two values
    SUB,      // Subtract two values
    MUL,      // Multiply two values
    DIV,      // Divide two values
    MOD,      // Find the remainder of two values
    INC,      // Increment the value by one
    DEC,      // Decrement the value by one

    // Data
    PUSH,     // Push value onto stack
    POP,      // Pop value from stack
    STORE,    // Store value in memory
    LOAD,     // Load value from memory

    // Jump Instructions
    JEQ,      // Jump if equal
    JNE,


    // IO
    IN,       // Gets input from the console and pushes it on to the stack
    PRINT,    // Print the last thing on the stack to the console
    PPRINT,   // Prints the last thing on the stack to the console and pops it

    // Miscellaneous 
    DEBUG,    // Displays the PC, stack and memory
    HALT,     // Halt execution
    NOOP,     // No operation
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
                    } else {
                        eprintln!("Error: Can't divide by zero!");
                    }
                }
            },
            Opcode::MOD => {
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    if b != 0 {
                        self.stack.push(a % b);
                    } else {
                        eprintln!("Error: Can't divide by zero!");
                    }
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
                    .expect("Error: Failed to read line");
                let a: i32 = input_line.trim().parse().expect("Error: Input not an integer");
                self.stack.push(a);
            }
            Opcode::PRINT => {
                if let Some(value) = self.stack.last() {
                    println!("{}", value);
                } else {
                    eprintln!("Error: Stack is empty");
                }
            },
            Opcode::PPRINT => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    eprintln!("Error: Stack is empty");
                }
            },
            Opcode::DEBUG => {
                self.debug_state();
            }
            Opcode::HALT => {
                self.running = false;
            },
            Opcode::NOOP => {
                // Does nothing
            },
        }
    }

    fn debug_state(&self) {
        println!("PC: {}, Stack: {:?}, Memory: {:?}", self.pc, self.stack, self.memory);
    }
}

impl VM {
    pub fn load_program_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);

        let mut program = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim(); // Trim whitespace

            // Skip comments + empty lines 
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Ok(value) = line.parse::<i32>() {
                program.push((Opcode::PUSH, Some(value)));
                continue;
            }

            let mut parts = line.split_whitespace();

            if let Some(opcode_str) = parts.next() {

                if line.trim().is_empty() || line.starts_with('#') {
                    continue;
                }

                let opcode = match opcode_str.to_uppercase().as_str() {
                    "ADD" => Opcode::ADD,
                    "SUB" => Opcode::SUB,
                    "MUL" => Opcode::MUL,
                    "DIV" => Opcode::DIV,
                    "MOD" => Opcode::MOD,
                    "INC" => Opcode::INC,
                    "DEC" => Opcode::DEC,
                    "PUSH" => Opcode::PUSH,
                    "POP" => Opcode::POP,
                    "STORE" => Opcode::STORE,
                    "LOAD" => Opcode::LOAD,
                    "IN" => Opcode::IN,
                    "DEBUG" => Opcode::DEBUG,
                    "PRINT" => Opcode::PRINT,
                    "PPRINT" => Opcode::PPRINT,
                    "HALT" => Opcode::HALT,
                    "NOOP" => Opcode::NOOP,
                    _ => {
                        eprintln!("Unknown opcode: {}", opcode_str);
                        continue;
                    }
                };

                let operand = parts.next().map(|s| s.parse::<i32>().ok()).flatten();
                program.push((opcode, operand));
            }
        }

        self.load_program(program);
        Ok(())
    }
}

fn main() {

    let mut vm = VM::new();

    vm.load_program_from_file("program.ovm").unwrap();

    if let Err(e) = vm.load_program_from_file("program.ovm") {
        eprintln!("Error loading program: {}", e);
        return;
    }

    vm.run();
}