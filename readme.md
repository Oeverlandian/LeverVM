Opcode Documentation
====================

Below is the list of opcodes supported by the virtual machine (VM), along with their descriptions and functionality.

Arithmetic Operations
---------------------

*   ```ADD```: Adds the two latest values on the stack.
*   ```SUB```: Subtracts the two latest values on the stack.
*   ```MUL```: Multiplies the two latest values on the stack.
*   ```DIV```: Divides the two latest values on the stack.
*   ```MOD```: Finds the remainder of the latest two values on the stack.
*   ```INC```: Increments the latest value on the stack by one.
*   ```DEC```: Decrements the latest value on the stack by one.

Stack Operations
---------------

*   ```PSH```: Pushes the given value onto the stack.
*   ```POP```: Pops the latest value from the stack.
*   ```DUP```: Duplicates the top value of the stack and pushes it onto the stack.
*   ```SWP```: Swaps the two top values on the stack.
*   ```CLR```: Clears the entire stack.

Memory Operations
---------------

*   ```STR {address(int)}```: Stores the latest value on the stack in memory to the given address.
*   ```LOA {address(int)}```: Loads the value at a given address from memory onto the stack.

Register Operations
---------------

*   ```SET {address(int)}```: Stores the latest value on the stack to the given register address (0-7).
*   ```GET {address(int)}```: Pushes the value in the given register address (0-7) to the stack.

Jump Operations
---------------

*   ```JMP {label}```: Unconditionally jumps to a specified label.
*   ```JEZ {label}```: Jumps to a label if the top value on the stack is equal to zero.
*   ```JNZ {label}```: Jumps to a label if the top value on the stack is not equal to zero.
*   ```JGZ {label}```: Jumps to a label if the top value on the stack is greater than zero.
*   ```JLZ {label}```: Jumps to a label if the top value on the stack is less than zero.

Input/Output Operations
-----------------------

*   ```INP```: Gets input from the console and pushes it onto the stack.
*   ```PRT```: Prints the top value on the stack to the console.
*   ```PPT```: Prints the top value on the stack and then pops it from the stack.

Miscellaneous Operations
------------------------

*   ```DEB```: Prints the current program counter (PC), stack, and memory state to the console.
*   ```HLT```: Halts the execution of the program.
*   ```NOP```: Does nothing (no operation).