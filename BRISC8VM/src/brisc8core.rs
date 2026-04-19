use crate::peripherals::Peripheral;

pub struct Brisc8core {
    memory: [u8; 0x100],
    registers: [u8; 0x04],
    peripherals: [Option<Box<dyn Peripheral>>; 0x100],
}

impl Brisc8core {
    pub fn new() -> Brisc8core {
        return Brisc8core {
            memory: [0; 0x100],
            registers: [0; 0x04],
            peripherals: [const { None }; 0x100],
        };
    }

    pub fn set_peripheral(&mut self, addr: u8, peripheral: Box<dyn Peripheral>) -> () {
        self.peripherals[addr as usize] = Some(peripheral);
    }

    pub fn set_memory(&mut self, addr: u8, data: u8) -> () {
        self.memory[addr as usize] = data;
    }

    pub fn do_cycle(&mut self) -> () {
        self.do_peripherals();
        self.do_instruction();
    }

    fn do_peripherals(&mut self) -> () {
        for x in &mut self.peripherals {
            if let Some(x) = x {
                x.do_cycle();
            }
        }
    }

    fn do_instruction(&mut self) -> () {
        let opcode = self.memory[self.registers[0x3] as usize];
        self.registers[0x3] += 1;

        let r_a = (opcode >> 2) & 0b0000_0011;
        let r_b = opcode & 0b0000_0011;
        let imm = ((opcode & 0b0011_0000) >> 2) | (opcode & 0b0000_0011);

        match opcode >> 4 {
            0b0000..=0b0011 => {
                // LIL #i, rA
                self.registers[r_a as usize] = (self.registers[r_a as usize] & 0b1111_0000) | imm;
            },
            0b0100..=0b0111 => {
                // LIH #i, rA
                self.registers[r_a as usize] = (imm << 4) | (self.registers[r_a as usize] & 0b0000_1111);
            },
            0b1000 => {
                // ADD rA, rB
                self.registers[r_a as usize] = self.registers[r_a as usize] | self.registers[r_b as usize];
            },
            0b1001 => {
                // NAND rA, rB
                self.registers[r_a as usize] = !(self.registers[r_a as usize] & self.registers[r_b as usize]);
            },
            0b1010 => {
                // IOR rA, rB
                let b_val = self.registers[r_b as usize];
                if let Some(peripheral) = &mut self.peripherals[b_val as usize] {
                    self.registers[r_a as usize] = peripheral.do_read(b_val);
                }
            },
            0b1011 => {
                // IOW rA, rB
                let b_val = self.registers[r_b as usize];
                if let Some(peripheral) = &mut self.peripherals[b_val as usize] {
                    peripheral.do_write(b_val, self.registers[r_a as usize]);
                }
            },
            0b1100 => {
                // LOA rA, rB
                self.registers[r_a as usize] = self.memory[self.registers[r_b as usize] as usize];
            },
            0b1101 => {
                // STO rA, rB
                self.memory[self.registers[r_b as usize] as usize] = self.registers[r_a as usize];
            },
            0b1110 => {
                // SKL rA, rB
                if (self.registers[r_a as usize] as i8) < (self.registers[r_b as usize] as i8)  {
                    self.registers[0x3 as usize] += 1;
                }
            },
            _ => {
                println!("Instruction not implemented {:?}", opcode >> 4);
                todo!();
            },
        }
    }
}