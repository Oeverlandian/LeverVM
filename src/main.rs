use std::collections::HashMap;
use std::io::BufRead;

const MAX_MEMORY_SIZE: usize = 1024 * 1024; // 1 MB
const REGISTER_AMOUNT: usize = 8;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    
    // Arithmetic 
    ADD, // Add's the two latest values on the stack
    SUB, // Subtracts the two latest values on the stack
    MUL, // Mulitplies the two latest values on the stack
    DIV, // Divides the two latest values on the stack
    MOD, // Finds the remainder of the latest two values on the stack
    INC, // Increment the latest value on the stack by one
    DEC, // Decrement the latest value on the stack by one

    // Stack Operations
    PSH, // Pushes the given value onto stack
    POP, // Pop the latest value from the stack
    DUP, // Duplicates the top of the stack and pushes it into the stack
    SWP, // Swaps the tow top elements on the stack
    CLR, // Clears the entire stack

    // Memory Operations
    STR, // Stores latest value on the stack in memory
    LOA, // Loads value at given adress from memory to the stack

    // Register Operations
    SET, // Sets the latest value on the stack to the specified register
    GET, // Pushes the value in the register to the stack

    // Jumps
    JMP, // Unconditional jump to label
    JEZ, // Jump if equal to zero to label
    JNZ, // Jump if not equal to zero to label
    JGZ, // Jump if greater than zero to label
    JLZ, // Jump if less than zero to label

    // IO
    INP, // Gets input from the console and pushes it on to the stack
    PRT, // Print the last thing on the stack to the console
    PPT, // Prints the last thing on the stack to the console and pops it
    PRC, // Prints the ASCII character on the top of the stack

    // Miscellaneous 
    DEB, // Prints the PC, stack and memory to the console
    HLT, // Halts execution of the program
    NOP, // No operation is executed
}

pub struct VM {
    stack: Vec<i32>,
    memory: HashMap<usize, i32>,
    registers: [i32; REGISTER_AMOUNT],
    program: Vec<(Opcode, Option<i32>)>,
    pc: usize,  // Program counter
    running: bool,
    labels: HashMap<String, usize>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            memory: HashMap::new(),
            registers: [0; REGISTER_AMOUNT],
            program: Vec::new(),
            pc: 0,
            running: false,
            labels: HashMap::new(),
        }
    }

    pub fn load_program(&mut self, program: Vec<(Opcode, Option<i32>)>) {
        self.program = program;
        self.pc = 0;
    }

    pub fn run(&mut self) {
        self.running = true;
        while self.running && self.pc < self.program.len() {
            let next_pc = self.execute_instruction();
            self.pc = next_pc;
        }
    }

    fn execute_instruction(&mut self) -> usize {
        let (opcode, operand) = self.program[self.pc];
        
        match opcode {
            Opcode::ADD => {
                if self.stack.len() < 2 {
                    eprintln!("Error: Stack underflow in ADD operation");
                    return self.pc + 1;
                }
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(a + b);
                }
                self.pc + 1
            },
            Opcode::SUB => {
                if self.stack.len() < 2 {
                    eprintln!("Error: Stack underflow in SUB operation");
                    return self.pc + 1;
                }
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(b - a);
                }
                self.pc + 1
            },
            Opcode::MUL => {
                if self.stack.len() < 2 {
                    eprintln!("Error: Stack underflow in MUL operation");
                    return self.pc + 1;
                }
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(a * b);
                }
                self.pc + 1
            },
            Opcode::DIV => {
                if self.stack.len() < 2 {
                    eprintln!("Error: Stack underflow in DIV operation");
                    return self.pc + 1;
                }
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    if b != 0 {
                        self.stack.push(b / a);
                    } else {
                        eprintln!("Error: Can't divide by zero!");
                    }
                }
                self.pc + 1
            },
            Opcode::MOD => {
                if self.stack.len() < 2 {
                    eprintln!("Error: Stack underflow in MOD operation");
                    return self.pc + 1;
                }
                if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                    if b != 0 {
                        self.stack.push(b % a);
                    } else {
                        eprintln!("Error: Can't divide by zero!");
                    }
                }
                self.pc + 1
            }
            Opcode::INC => {
                if let Some(a) = self.stack.pop() {
                    self.stack.push(a + 1);
                } else {
                    eprintln!("Error: Stack underflow in INC operation");
                }
                self.pc + 1
            }
            Opcode::DEC => {
                if let Some(a) = self.stack.pop() {
                    self.stack.push(a - 1);
                } else {
                    eprintln!("Error: Stack underflow in DEC operation");
                }
                self.pc + 1
            }
            Opcode::PSH => {
                if let Some(value) = operand {
                    self.stack.push(value);
                }
                self.pc + 1
            },
            Opcode::POP => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack is empty, can't pop");
                } else {
                    self.stack.pop();
                }
                self.pc + 1
            }
            Opcode::STR => {
                if let (Some(value), Some(address)) = (self.stack.pop(), operand) {
                    if address >= 0 && (address as usize) < MAX_MEMORY_SIZE {
                        self.memory.insert(address as usize, value);
                    } else {
                        eprintln!("Error: Memory address out of bounds");
                    }
                }
                self.pc + 1
            },
            Opcode::LOA => {
                if let Some(address) = operand {
                    if let Some(&value) = self.memory.get(&(address as usize)) {
                        self.stack.push(value);
                    }
                }
                self.pc + 1
            },
            Opcode::DUP => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack Underflow");
                } else {
                    if let Some(a) = self.stack.pop() {
                        let b = a;
                        self.stack.push(a);
                        self.stack.push(b);
                    }
                }
                self.pc + 1
            },
            Opcode::SWP => {
                if self.stack.len() < 2 {
                    eprintln!("Error: Stack Underflow");
                } else {
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                }
                self.pc + 1
            },
            Opcode::CLR => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack is already empty!");
                } else {
                    self.stack.clear();
                }
                self.pc + 1
            },
            Opcode::SET => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack Underflow");
                } else {
                    if let Some(reg) = operand {
                        self.registers[reg as usize] = self.stack.pop().unwrap_or(0);
                    }
                }
                self.pc + 1
            },
            Opcode::GET => {
                if let Some(reg) = operand {
                    let value = self.registers[reg as usize]; 
                    self.stack.push(value);
                }
                self.pc + 1
            }
            Opcode::INP => {
                let mut input_line = String::new();
                std::io::stdin()
                    .read_line(&mut input_line)
                    .expect("Error: Failed to read line");
                let a: i32 = match input_line.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("Error: Input is not a valid integer");
                        return self.pc + 1;
                    }
                };
                self.stack.push(a);
                self.pc + 1
            },
            Opcode::PRT => {
                if let Some(value) = self.stack.last() {
                    println!("{}", value);
                } else {
                    eprintln!("Error: Stack is empty");
                }
                self.pc + 1
            },
            Opcode::PPT => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    eprintln!("Error: Stack is empty");
                }
                self.pc + 1
            },
            Opcode::PRC => {
                if let Some(value) = self.stack.pop() {
                    if let Some(ch) = char::from_u32(value as u32) {
                        print!("{}", ch);
                    } else {
                        eprintln!("Error: Invalid ASCII code {}", value);
                    }
                } else {
                    eprintln!("Error: Stack is empty, can't print character");
                }
                self.pc + 1
            }
            Opcode::DEB => {
                self.debug_state();
                self.pc + 1
            }
            Opcode::HLT => {
                self.running = false;
                self.pc + 1
            },
            Opcode::NOP => {
                // Does nothing
                self.pc + 1
            },
            Opcode::JMP => {
                if let Some(target) = operand {
                    if (target as usize) < self.program.len() {
                        return target as usize;
                    } else {
                        eprintln!("Error: Invalid jump target '{}'", target);
                    }
                }
                self.pc + 1
            }
            Opcode::JEZ => {
                if let Some(&value) = self.stack.last() {
                    if value == 0 {
                        if let Some(target) = operand {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}'", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::JNZ => {
                if let Some(&value) = self.stack.last() {
                    if value != 0 {
                        if let Some(target) = operand {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}'", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::JGZ => {
                if let Some(&value) = self.stack.last() {
                    if value > 0 {
                        if let Some(target) = operand {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}'", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::JLZ => {
                if let Some(&value) = self.stack.last() {
                    if value < 0 {
                        if let Some(target) = operand {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}'", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            
        }
    }

    fn debug_state(&self) {
        println!("PC: {}, Stack: {:?}, Memory: {:?}, Registers: {:?}, Labels: {:?}", self.pc, self.stack, self.memory, self.registers, self.labels);
    }
}

impl VM {
    pub fn load_program_from_file(&mut self, filename: &str) -> std::io::Result<()> {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);
        let mut program = Vec::new();
        
        // First pass: collect all labels and their positions
        let mut current_position = 0;
        let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
        
        for line in &lines {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Check for label definition (ends with ':')
            if line.ends_with(':') {
                let label = line[..line.len()-1].trim().to_string();
                self.labels.insert(label, current_position);
                continue;
            }            
            
            // Count instruction
            if !line.trim().is_empty() {
                current_position += 1;
            }
        }
        
        // Second pass: process instructions
        current_position = 0;
        for line in lines {
            let line = line.trim();
            
            // Skip comments, empty lines, and labels
            if line.is_empty() || line.starts_with('#') || line.ends_with(':') {
                continue;
            }
            
            // Parse instruction
            let mut parts = line.split_whitespace();
            if let Some(opcode_str) = parts.next() {
                let opcode = match opcode_str.to_uppercase().as_str() {
                    "ADD" => Opcode::ADD,
                    "SUB" => Opcode::SUB,
                    "MUL" => Opcode::MUL,
                    "DIV" => Opcode::DIV,
                    "MOD" => Opcode::MOD,
                    "INC" => Opcode::INC,
                    "DEC" => Opcode::DEC,
                    "PSH" => Opcode::PSH,
                    "POP" => Opcode::POP,
                    "STR" => Opcode::STR,
                    "LOA" => Opcode::LOA,
                    "DUP" => Opcode::DUP,
                    "SWP" => Opcode::SWP,
                    "CLR" => Opcode::CLR,
                    "SET" => Opcode::SET,
                    "GET" => Opcode::GET,
                    "INP" => Opcode::INP,
                    "PRT" => Opcode::PRT,
                    "PPT" => Opcode::PPT,
                    "PRC" => Opcode::PRC,
                    "DEB" => Opcode::DEB,
                    "HLT" => Opcode::HLT,
                    "NOP" => Opcode::NOP,
                    "JMP" => Opcode::JMP,
                    "JEZ" => Opcode::JEZ,
                    "JNZ" => Opcode::JNZ,
                    "JGZ" => Opcode::JGZ,
                    "JLZ" => Opcode::JLZ,
                    _ => {
                        eprintln!("Unknown opcode: {}", opcode_str);
                        continue;
                    }
                };

                let operand = if let Some(operand_str) = parts.next() {
                    if self.labels.contains_key(operand_str) {
                        Some(*self.labels.get(operand_str).unwrap() as i32)
                    } else {
                        operand_str.parse().ok()
                    }
                } else {
                    None
                };
                

                program.push((opcode, operand));
                current_position += 1;
            }
        }

        self.load_program(program);
        Ok(())
    }
}

fn main() {
    let mut vm = VM::new();
    if let Err(e) = vm.load_program_from_file("program.vm") {
        eprintln!("Error loading program: {}", e);
        return;
    }
    vm.run();
}