# BRISC8
Simple 8-bit RISC architecure based on BRISC16.

## Registers
The CPU uses 2 bits for register indexing, thus we have 4 registers. Register 3 is reserved for the
program counter. In assembly and documentation they a referred to a %R0-%R3.

### Calling Convention
| Register | Use             |
|----------|-----------------|
| %R0      | First Operand   |
| %R1      | Second Operand  |
| %R2      | Return Address  |
| %R3      | Program Counter |

## Instructions
There are 10 basic instructions following two decoding modes
### Operand Decoding Modes
#### Register Mode:
```
0bOOOO AABB         
  ┃    ┃ ┗ Register B
  ┃    ┗ Register A
  ┗ Opcode
```

#### Immediate Mode:
```
    ┏━━━━┳ 4-bit Immediate
0bOOii AAii         
  ┃    ┗ Register A
  ┗ Opcode
```

### Register Rationale
Generally rA should be the destination of any operation taking place within the CPU. If we're using a
register as an address that should be rB.

### Instruction Table
| Opcode   | Decode Mode | Instruction     | C Reference                   | Notes                       |
|----------|-------------|-----------------|-------------------------------|-----------------------------|
| `0b00ii` | Immediate   | `LIL #i, %rA`   | `rA = i`                      | Clears top 4 bits of rA     |
| `0b01ii` | Immediate   | `LIH #i, %rA`   | `rA = (rA & 0xF) \| (i << 4)` |                             |
| `0b1000` | Register    | `ADD %rA, %rB`  | `rA = rA + rB`                |                             |
| `0b1001` | Register    | `NAND %rA, %rB` | `rA = !(rA & rB)`             |                             |
| `0b1010` | Register    | `IOR %rA, %rB`  | `rA = peripheral[rB]`         | Read from peripheral rB     |
| `0b1011` | Register    | `IOW %rA, %rB`  | `peripheral[rB] = rA`         | Write rA to peripheral rB   |
| `0b1100` | Register    | `LOA %rA, %rB`  | `rA = mem[rB]`                | Load from memory rB         |
| `0b1101` | Register    | `STO %rA, %rB`  | `mem[rB] = rA`                | Write rA to memory rB       |
| `0b1110` | Register    | `SKL %rA, %rB`  | `rA < rB ? PC++ : pass`       | Skip instruction if rA < rB |
| `0b1111` | Register    | `SWP %rA, %rB`  | `rA, rB = rB, rA`             |                             |
