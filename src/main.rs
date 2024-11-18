use std::collections::HashMap;
use std::io::BufRead;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_MEMORY_SIZE: usize = 1024 * 1024; // 1 MB
const REGISTER_AMOUNT: usize = 8;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    
    // Arithmetic 
    ADD, // Add's the two latest values on the stack, if there are two operands it adds those two provided registers and pushes it onto the stack
    SUB, // Subtracts the two latest values on the stack, if there are two operands it subtracts the second provided register from the first provided register and pushes it onto the stack
    MUL, // Mulitplies the two latest values on the stack, if there are two operands it multiplies those two provided registers and pushes it onto the stack
    DIV, // Divides the two latest values on the stack, if there are two operands it divides the first provided register from the second provided and pushes it onto the stack
    MOD, // Finds the remainder of the latest two values on the stack, if there are two operands it finds the remainder of the two provided registers and pushes it onto the stack
    INC, // Increment the latest value on the stack by one, if an operand is provided it increments the register
    DEC, // Decrement the latest value on the stack by one, if an operand is provided it decrements the register

    // Stack Operations
    PSH, // Pushes the given value onto stack
    POP, // Pop the latest value from the stack
    DUP, // Duplicates the top of the stack and pushes it into the stack
    SWP, // Swaps the tow top elements on the stack
    SCL, // Clears the entire stack

    // Memory Operations
    STR, // Stores latest value on the stack in memory
    LOA, // Loads value at given adress from memory to the stack
    MCL, // Clears the entire heap

    // Register Operations
    MOV, // Moves a value from one register to another
    COP, // Copies a value from one register to another
    SET, // Sets the latest value on the stack to the specified register
    GET, // Pushes the value in the register to the stack

    // Jumps
    JMP, // Unconditional jump to label
    JEZ, // Jump if equal to zero to label
    JNZ, // Jump if not equal to zero to label
    JGZ, // Jump if greater than zero to label
    JLZ, // Jump if less than zero to label

    // Comparison Operations
    EQU, // Push 1 if top two values are equal, 0 otherwise. If there are two operands it compares the two given registers and returns 1 if equal, 0 otherwise
    NEQ, // Push 1 if top two values are not equal, 0 otherwise. If there are two operands it compares the two given registers and returns 1 if not equal, 0 otherwise
    GTH, // Push 1 if second-to-top > top, 0 otherwise. If there are two operands it compares the two given registers and returns 1 if the first provided register is greater than the second provided register, 0 otherwise
    LTH, // Push 1 if second-to-top < top, 0 otherwise  If there are two operands it compares the two given registers and returns 1 if the first provided register is less than the second provided register, 0 otherwise
    GTE, // Push 1 if second-to-top >= top, 0 otherwise. If there are two operands it compares the two given registers and returns 1 if the first provided register is greater than or equal than the second provided register, 0 otherwise
    LTE, // Push 1 if second-to-top <= top, 0 otherwise. If there are two operands it compares the two given registers and returns 1 if the first provided register is less than or equal than the second provided register, 0 otherwise

    // IO
    INP, // Gets input from the console and pushes it on to the stack
    PRT, // Print the last thing on the stack to the console
    PPT, // Prints the last thing on the stack to the console and pops it
    PRC, // Prints the ASCII character on the top of the stack

    // Miscellaneous 
    TIM, // Pushes the amount of epoch seconds to the stack
    DEB, // Prints the PC, stack and memory to the console
    HLT, // Halts execution of the program
    NOP, // No operation is executed
}

pub struct VM {
    stack: Vec<i32>,
    memory: HashMap<usize, i32>,
    registers: [i32; REGISTER_AMOUNT],
    program: Vec<(Opcode, Option<i32>, Option<i32>)>,
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

    pub fn load_program(&mut self, program: Vec<(Opcode, Option<i32>, Option<i32>)>) {
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
        let (opcode, operand_1, operand_2) = self.program[self.pc];
        
        match opcode {
            Opcode::ADD => {
                if let Some(operand_2) = operand_2 { // Use register ADD if there is a second operand
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result = self.registers[operand_1.unwrap_or(0) as usize] + self.registers[operand_2 as usize];
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in ADD operation!");
                    }
                } else { // Otherwise use stack ADD
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack underflow in ADD operation!");
                        return self.pc + 1;
                    }
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a + b);
                    }
                }
                self.pc + 1
            },
            Opcode::SUB => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result = self.registers[operand_1.unwrap_or(0) as usize] - self.registers[operand_2 as usize];
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in SUB operation!")
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack underflow in SUB operation!");
                        return self.pc + 1;
                    }
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(b - a);
                    }
                }
                self.pc + 1
            },
            Opcode::MUL => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result = self.registers[operand_1.unwrap_or(0) as usize] * self.registers[operand_2 as usize];
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in MUL operation!")
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack underflow in MUL operation!");
                        return self.pc + 1;
                    }
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(a * b);
                    }
                }   
                self.pc + 1
            },
            Opcode::DIV => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result = self.registers[operand_1.unwrap_or(0) as usize] / self.registers[operand_2 as usize];
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in DIV operation!")
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack underflow in DIV operation!");
                        return self.pc + 1;
                    }
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if b != 0 {
                            self.stack.push(b / a);
                        } else {
                            eprintln!("Error: Can't divide by zero in DIV operation!");
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::MOD => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result = self.registers[operand_1.unwrap_or(0) as usize] % self.registers[operand_2 as usize];
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in MOD operation!")
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack underflow in MOD operation!");
                        return self.pc + 1;
                    }
                    if let (Some(b), Some(a)) = (self.stack.pop(), self.stack.pop()) {
                        if b != 0 {
                            self.stack.push(b % a);
                        } else {
                            eprintln!("Error: Can't divide by zero in MOD operation!");
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::INC => {
                if let Some(register) = operand_1 {
                    self.registers[register as usize] += 1;
                } else {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push(a + 1);
                    } else {
                        eprintln!("Error: Stack underflow in INC operation!");
                    }
                }
                self.pc + 1
            },
            Opcode::DEC => {
                if let Some(register) = operand_1 {
                    self.registers[register as usize] -= 1;
                } else {
                    if let Some(a) = self.stack.pop() {
                        self.stack.push(a - 1);
                    } else {
                        eprintln!("Error: Stack underflow in DEC operation!");
                    }
                }
                self.pc + 1
            },
            Opcode::PSH => {
                if let Some(value) = operand_1 {
                    self.stack.push(value);
                }
                self.pc + 1
            },
            Opcode::POP => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack is empty, can't pop using POP operation!");
                } else {
                    self.stack.pop();
                }
                self.pc + 1
            },
            Opcode::STR => {
                if let (Some(value), Some(address)) = (self.stack.pop(), operand_1) {
                    if address >= 0 && (address as usize) < MAX_MEMORY_SIZE {
                        self.memory.insert(address as usize, value);
                    } else {
                        eprintln!("Error: Memory address out of bounds in STR operation!");
                    }
                }
                self.pc + 1
            },
            Opcode::LOA => {
                if let Some(address) = operand_1 {
                    if let Some(&value) = self.memory.get(&(address as usize)) {
                        self.stack.push(value);
                    }
                }
                self.pc + 1
            },
            Opcode::DUP => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack Underflow in DUP operation!");
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
                    eprintln!("Error: Stack Underflow in SWP operation!");
                } else {
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(b);
                        self.stack.push(a);
                    }
                }
                self.pc + 1
            },
            Opcode::SCL => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack is already empty, can't perform SCL operation!");
                } else {
                    self.stack.clear();
                }
                self.pc + 1
            },
            Opcode::SET => {
                if self.stack.is_empty() {
                    eprintln!("Error: Stack Underflow in SET operation!");
                } else {
                    if let Some(reg) = operand_1 {
                        self.registers[reg as usize] = self.stack.pop().unwrap_or(0);
                    }
                }
                self.pc + 1
            },
            Opcode::GET => {
                if let Some(reg) = operand_1 {
                    let value = self.registers[reg as usize]; 
                    self.stack.push(value);
                }
                self.pc + 1
            }
            Opcode::INP => {
                let mut input_line = String::new();
                std::io::stdin()
                    .read_line(&mut input_line)
                    .expect("Error: Failed to read line in INP operation!");
                let a: i32 = match input_line.trim().parse() {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("Error: Input is not a valid integer in INP operation!");
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
                    eprintln!("Error: Stack is empty in PRT operation!");
                }
                self.pc + 1
            },
            Opcode::PPT => {
                if let Some(value) = self.stack.pop() {
                    println!("{}", value);
                } else {
                    eprintln!("Error: Stack is empty in PPT operation!");
                }
                self.pc + 1
            },
            Opcode::PRC => {
                if let Some(value) = self.stack.pop() {
                    if let Some(ch) = char::from_u32(value as u32) {
                        print!("{}", ch);
                    } else {
                        eprintln!("Error: Invalid ASCII code {} in PRC operation!", value);
                    }
                } else {
                    eprintln!("Error: Stack is empty, can't print character using PRC operation!");
                }
                self.pc + 1
            },
            Opcode::DEB => {
                self.debug_state();
                self.pc + 1
            },
            Opcode::HLT => {
                self.running = false;
                self.pc + 1
            },
            Opcode::NOP => {
                // Does nothing
                self.pc + 1
            },
            Opcode::JMP => {
                if let Some(target) = operand_1 {
                    if (target as usize) < self.program.len() {
                        return target as usize;
                    } else {
                        eprintln!("Error: Invalid jump target '{}' in JMP operation!", target);
                    }
                }
                self.pc + 1
            },
            Opcode::JEZ => {
                if let Some(&value) = self.stack.last() {
                    if value == 0 {
                        if let Some(target) = operand_1 {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}' in JEZ operation!", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::JNZ => {
                if let Some(&value) = self.stack.last() {
                    if value != 0 {
                        if let Some(target) = operand_1 {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}' in JNZ operation!", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::JGZ => {
                if let Some(&value) = self.stack.last() {
                    if value > 0 {
                        if let Some(target) = operand_1 {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}' in JGZ operation!", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::JLZ => {
                if let Some(&value) = self.stack.last() {
                    if value < 0 {
                        if let Some(target) = operand_1 {
                            if let Some(&resolved_target) = self.labels.get(&target.to_string()) {
                                return resolved_target;
                            } else if (target as usize) < self.program.len() {
                                return target as usize;
                            } else {
                                eprintln!("Error: Invalid jump target '{}' in JLZ operation!", target);
                            }
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::EQU => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result;
                        if (operand_1.unwrap_or(0) as usize) == (operand_2 as usize) {
                            result = 1;
                        } else {
                            result = 0;
                        }
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in EQU operation!");
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack Underflow in EQU operation!");
                        return self.pc + 1;
                    }
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if a == b {
                            self.stack.push(1);
                        } else {
                            self.stack.push(0);
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::NEQ => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result;
                        if (operand_1.unwrap_or(0) as usize) != (operand_2 as usize) {
                            result = 1;
                        } else {
                            result = 0;
                        }
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in NEQ operation!");
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack Underflow in NEQ operation!");
                        return self.pc + 1;
                    }
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if a != b {
                            self.stack.push(0);
                        } else {
                            self.stack.push(1);
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::GTH => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result;
                        if (operand_1.unwrap_or(0) as usize) > (operand_2 as usize) {
                            result = 1;
                        } else {
                            result = 0;
                        }
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in GTH operation!");
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack Underflow in GTH operation!");
                        return self.pc + 1;
                    }
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if a < b {
                            self.stack.push(1);
                        } else {
                            self.stack.push(0);
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::LTH => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result;
                        if (operand_1.unwrap_or(0) as usize) < (operand_2 as usize) {
                            result = 1;
                        } else {
                            result = 0;
                        }
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in LTH operation!");
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack Underflow in LTH operation!");
                        return self.pc + 1;
                    }
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if a > b {
                            self.stack.push(1);
                        } else {
                            self.stack.push(0);
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::GTE => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result;
                        if (operand_1.unwrap_or(0) as usize) >= (operand_2 as usize) {
                            result = 1;
                        } else {
                            result = 0;
                        }
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in GTE operation!");
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack Underflow in GTE operation!");
                        return self.pc + 1;
                    }
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if a <= b {
                            self.stack.push(1);
                        } else {
                            self.stack.push(0);
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::LTE => {
                if let Some(operand_2) = operand_2 {
                    if (operand_1.unwrap_or(0) as usize) < REGISTER_AMOUNT && (operand_2 as usize) < REGISTER_AMOUNT {
                        let result;
                        if (operand_1.unwrap_or(0) as usize) <= (operand_2 as usize) {
                            result = 1;
                        } else {
                            result = 0;
                        }
                        self.stack.push(result);
                    } else {
                        eprintln!("Error: Invalid register index in LTH operation!");
                    }
                } else {
                    if self.stack.len() < 2 {
                        eprintln!("Error: Stack Underflow");
                        return self.pc + 1;
                    }
                    if let (Some(a), Some(b)) = (self.stack.pop(), self.stack.pop()) {
                        if a >= b {
                            self.stack.push(1);
                        } else {
                            self.stack.push(0);
                        }
                    }
                }
                self.pc + 1
            },
            Opcode::MCL => {
                if self.memory.is_empty() {
                    eprintln!("Error: Memory is already clear, can't perform MCL operation!")
                } else {
                    self.memory.clear();   
                }

                return self.pc + 1
            },
            Opcode::TIM => {
                let now = SystemTime::now();
                let duration_since_epoch = now.duration_since(UNIX_EPOCH)
                .expect("Time went backwards in TIM operation!");
            
                self.stack.push(duration_since_epoch.as_secs() as i32);

                return self.pc + 1
            },
            Opcode::MOV => {
                if let Some(operand_2) = operand_2 {
                    let operand_1 = operand_1.unwrap_or(0);
                    let value= self.registers[operand_1 as usize];

                    self.registers[operand_1 as usize] = 0;
                    self.registers[operand_2 as usize] = value;
                } else {
                    eprintln!("Not enough operands provided in MOV operation!")
                }
                return self.pc + 1
            }
            Opcode::COP => {
                if let Some(operand_2) = operand_2 {
                    let operand_1 = operand_1.unwrap_or(0);
                    let value= self.registers[operand_1 as usize];

                    self.registers[operand_2 as usize] = value;
                } else {
                    eprintln!("Not enough operands provided in MOV operation!")
                }

               return self.pc + 1
            }
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
                    "SCL" => Opcode::SCL,
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
                    "EQU" => Opcode::EQU,
                    "NEQ" => Opcode::NEQ,
                    "GTH" => Opcode::GTH,
                    "LTH" => Opcode::LTH,
                    "GTE" => Opcode::GTE,
                    "LTE" => Opcode::LTE,
                    "MCL" => Opcode::MCL,
                    "TIM" => Opcode::TIM,
                    "MOV" => Opcode::MOV,
                    "COP" => Opcode::COP,
                    _ => {
                        eprintln!("Unknown opcode: {}", opcode_str);
                        continue;
                    }
                };

                let operand_1 = if let Some(operand_str) = parts.next() {
                    if self.labels.contains_key(operand_str) {
                        Some(*self.labels.get(operand_str).unwrap() as i32)
                    } else {
                        operand_str.parse().ok()
                    }
                } else {
                    None
                };

                let operand_2 = parts.next().and_then(|s| s.parse::<i32>().ok());
                

                program.push((opcode, operand_1, operand_2));
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