# VM Opcode Documentation

## Arithmetic Operations

* ```ADD [register1] [register2]``` 
  - Without operands: Adds the two latest values on the stack
  - With 2 registers: Adds values in specified registers, pushes result to stack

* ```SUB [register1] [register2]```
  - Without operands: Subtracts the two latest values on the stack
  - With 2 registers: Subtracts values in specified registers, pushes result to stack

* ```MUL [register1] [register2]```
  - Without operands: Multiplies the two latest values on the stack
  - With 2 registers: Multiplies values in specified registers, pushes result to stack

* ```DIV [register1] [register2]```
  - Without operands: Divides the two latest values on the stack
  - With 2 registers: Divides values in specified registers, pushes result to stack

* ```MOD [register1] [register2]```
  - Without operands: Finds the remainder of the latest two values on the stack
  - With 2 registers: Finds remainder of values in specified registers, pushes result to stack

* ```INC [register]```
  - Without operand: Increments the latest value on the stack by one
  - With register: Increments the specified register by one

* ```DEC [register]```
  - Without operand: Decrements the latest value on the stack by one
  - With register: Decrements the specified register by one

## Stack Operations

* ```PSH value``` 
  - Pushes the given value onto the stack

* ```POP```
  - Removes the latest value from the stack

* ```DUP```
  - Duplicates the top value of the stack and pushes it onto the stack

* ```SWP```
  - Swaps the two top values on the stack

* ```SCL```
  - Clears the entire stack

## Memory Operations

* ```STR [address]```
  - Stores the latest value on the stack in memory at the specified address

* ```LOA [address]```
  - Loads the value at the given address from memory onto the stack

* ```MCL```
  - Clears the entire heap/memory

## Register Operations

* ```MOV [source_register] [destination_register]```
  - Moves value from source register to destination register, zeroing the source

* ```COP [source_register] [destination_register]```
  - Copies value from source register to destination register

* ```SET [register]```
  - Stores the latest value on the stack to the specified register

* ```GET [register]```
  - Pushes the value in the specified register to the stack

## Jump Operations

* ```JMP [label/address]```
  - Unconditionally jumps to a specified label or program address

* ```JEZ [label/address]```
  - Jumps to a label or address if the top stack value is zero

* ```JNZ [label/address]```
  - Jumps to a label or address if the top stack value is not zero

* ```JGZ [label/address]```
  - Jumps to a label or address if the top stack value is greater than zero

* ```JLZ [label/address]```
  - Jumps to a label or address if the top stack value is less than zero

## Comparison Operations

* ```EQU [register1] [register2]```
  - Without operands: Pushes 1 if stack's top two values are equal, otherwise 0
  - With registers: Compares register values, pushes 1 if equal, otherwise 0

* ```NEQ [register1] [register2]```
  - Without operands: Pushes 1 if stack's top two values are NOT equal, otherwise 0
  - With registers: Compares register values, pushes 1 if not equal, otherwise 0

* ```GTH [register1] [register2]```
  - Without operands: Pushes 1 if second-to-top value is greater than top value, otherwise 0
  - With registers: Compares register values, pushes 1 if first register is greater, otherwise 0

* ```LTH [register1] [register2]```
  - Without operands: Pushes 1 if second-to-top value is less than top value, otherwise 0
  - With registers: Compares register values, pushes 1 if first register is less, otherwise 0

* ```GTE [register1] [register2]```
  - Without operands: Pushes 1 if second-to-top value is greater than or equal to top value, otherwise 0
  - With registers: Compares register values, pushes 1 if first register is greater or equal, otherwise 0

* ```LTE [register1] [register2]```
  - Without operands: Pushes 1 if second-to-top value is less than or equal to top value, otherwise 0
  - With registers: Compares register values, pushes 1 if first register is less or equal, otherwise 0

## Input/Output Operations

* ```INP```
  - Gets input from the console and pushes it onto the stack

* ```PRT```
  - Prints the top value on the stack to the console

* ```PPT```
  - Prints the top value on the stack and then pops it from the stack

* ```PRC```
  - Prints an ASCII character based on the value at the top of the stack

## Miscellaneous Operations

* ```TIM```
  - Pushes the current time in Epoch Seconds to the stack

* ```DEB```
  - Prints the current program counter (PC), stack, memory state, registers states, and labels to the console

* ```HLT```
  - Halts the execution of the program

* ```NOP```
  - Does nothing (no operation)

## Notes
- Registers are 0-indexed (0-7)
- Some operations have dual functionality with or without register operands