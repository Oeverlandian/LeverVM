Opcode Documentation
====================

Below is the list of opcodes supported by the virtual machine (VM), along with their descriptions and functionality.

Arithmetic Operations
---------------------

*   ADD: Adds the two latest values on the stack.
*   SUB: Subtracts the two latest values on the stack.
*   MUL: Multiplies the two latest values on the stack.
*   DIV: Divides the two latest values on the stack.
*   MOD: Finds the remainder of the latest two values on the stack.
*   INC: Increments the latest value on the stack by one.
*   DEC: Decrements the latest value on the stack by one.

Data Operations
---------------

*   PSH: Pushes the given value onto the stack.
*   POP: Pops the latest value from the stack.
*   STR: Stores the latest value on the stack in memory.
*   LOA: Loads the value at a given address from memory onto the stack.
*   DUP: Duplicates the top value of the stack and pushes it onto the stack.

Jump Operations
---------------

*   JMP: Unconditionally jumps to a specified label.
*   JEZ: Jumps to a label if the top value on the stack is equal to zero.
*   JNZ: Jumps to a label if the top value on the stack is not equal to zero.
*   JGZ: Jumps to a label if the top value on the stack is greater than zero.
*   JLZ: Jumps to a label if the top value on the stack is less than zero.

Input/Output Operations
-----------------------

*   INP: Gets input from the console and pushes it onto the stack.
*   PRT: Prints the top value on the stack to the console.
*   PPT: Prints the top value on the stack and then pops it from the stack.

Miscellaneous Operations
------------------------

*   DEB: Prints the current program counter (PC), stack, and memory state to the console.
*   HLT: Halts the execution of the program.
*   NOP: Does nothing (no operation).