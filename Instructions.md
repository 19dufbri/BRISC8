| Bit Pattern |   Mnemonic   | Description                             |
|-------------|--------------|-----------------------------------------|
| 0b0000 iiii | LIL #i		 | Load 4 bits into the lower half of r0   |
| 0b0001 iiii | LIH #i		 | Load 4 bits into the high half of r0    |
| 0b0010 rr00 | PSH rA		 | Push rA onto the stack                  |
| 0b0010 xx10 | PSH PC		 | Push the Program Counter onto the stack |
| 0b0010 xx11 | PSH SP		 | Push the Stack Pointer onto the stack   |
| 0b0011 rr00 | POP rA		 | Pop rA from the stack                   |
| 0b0011 xx10 | POP PC		 | Pop the Program Counter from the stack  |
| 0b0011 xx10 | POP SP		 | Pop the Stack Pointer from the stack    |
| 0b0100 rrrr | MOV PC, rArB | Move the PC to the register pair rArB   |
| 0b0101 rrrr | MOV SP, rArB | Move the SP to the register pair rArB   |
| 0b0110 rrrr | MOV rArB, PC | Move the register pair rArB to the PC   |
| 0b0111 rrrr | MOV rArB, SP | Move the register pair rArB to the SP   |
| 0b1000 rrrr | ADD rA, rB	 | Add rA to rB and store the result in rB |
| 0b1001 rrrr | NOR rA, rB	 | NOR rA to rB and store the result in rB |
| 0b1010 rrrr | IOR rA, rB   | Read IO addr rA into rB                 |
| 0b1011 rrrr | IOW rA, rB   | Store rB into IO addr rA                |
| 0b1100 rrrr | LOA rA, rB	 | Load memory[rA] into rB                 |
| 0b1101 rrrr | STO rA, rB	 | Store rB into memory[rA]                |
| 0b1110 rrrr | SKL rA, rB	 | Skip the next instruction if rA < rB    |
| 0b1111 rrrr | MOV rA, rB   | Copy rA to rB                           |