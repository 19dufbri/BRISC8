use std::{collections::HashMap, mem::take};

use anyhow::{Result, anyhow};

use crate::lexer::{InstructionArg, Token};

pub struct Generator {
    output: Vec<u8>,
    labels: HashMap<String, u8>,
    fixups: Vec<Fixup>,
}

struct Fixup {
    offset: usize,
    label: String,
    nibble: Nibble,  // Low | High
}

enum Nibble { Low, High }

impl Generator {
    pub fn new() -> Generator {
        Generator {
            output: Vec::new(),
            labels: HashMap::new(),
            fixups: Vec::new()
        }
    }

    pub fn assemble(&mut self, tokens: Vec<Token>) -> Result<Vec<u8>> {
        for tok in tokens {
            match tok {
                Token::Label(name) => { self.labels.insert(name, self.output.len() as u8); }
                Token::Instruction(op, args) => self.emit_instruction(&op, args)?,
                Token::Byte(b) => self.output.push(b),
                Token::Directive(_) => panic!("Directives not implemented"),
            }
        }
        self.resolve_fixups()?;
        Ok(take(&mut self.output))
    }

    fn resolve_fixups(&mut self) -> Result<()> {
        for fixup in &self.fixups {
            let mut instruction = self.output[fixup.offset];
            let label = self.labels[&fixup.label];
            instruction &= 0b11001100;
            instruction |= match fixup.nibble {
                Nibble::Low => {
                    let nibble = label & 0x0F;
                    ((nibble & 0b1100) << 2) | (nibble & 0b0011)
                },
                Nibble::High => {
                    let nibble = (label & 0xF0) >> 4;
                    ((nibble & 0b1100) << 2) | (nibble & 0b0011)
                },
            };
            self.output[fixup.offset] = instruction;
        }
        Ok(())
    }

    fn emit_instruction(&mut self, op: &str, args: Vec<InstructionArg>) -> Result<()> {
        match (op.to_uppercase().as_str(), args.as_slice()) {
            ("LIL",  [imm_or_label, InstructionArg::Register(a)]) => {
                let val = self.encode_imm_or_label(0b00, *a, imm_or_label, Nibble::Low);
                self.push(val)
            },
            ("LIH",  [imm_or_label, InstructionArg::Register(a)]) => {
                let val = self.encode_imm_or_label(0b01, *a, imm_or_label, Nibble::High);
                self.push(val)
            },
            ("LIR",  [imm_or_label, InstructionArg::Register(a)]) => {
                let val = self.encode_imm_or_label(0b00, *a, imm_or_label, Nibble::Low);
                self.push(val);
                let val = self.encode_imm_or_label(0b01, *a, imm_or_label, Nibble::High);
                self.push(val);
            },
            ("ADD",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1000, *a, *b)),
            ("NAND", [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1001, *a, *b)),
            ("IOR",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1010, *a, *b)),
            ("IOW",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1011, *b, *a)),
            ("LOA",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1100, *a, *b)),
            ("STO",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1101, *b, *a)),
            ("SKL",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1110, *a, *b)),
            ("SWP",  [InstructionArg::Register(a), InstructionArg::Register(b)]) => self.push(encode_reg(0b1111, *a, *b)),
            _ => return Err(anyhow!("unknown instruction or bad args: {op}"))
        }
        Ok(())
    }

    fn push(&mut self, value: u8) {
        self.output.push(value);
    }
    
    fn encode_imm_or_label(&mut self, opcode: u8, r_a: u8, arg: &InstructionArg, nibble: Nibble) -> u8 {
        return match arg {
            InstructionArg::Immediate(i) => encode_imm(opcode, r_a, *i, nibble),
            InstructionArg::Label(label) => {
                self.fixups.push(Fixup { offset: self.output.len(), label: label.into(), nibble });
                encode_imm(opcode, r_a, 0, Nibble::Low)
            },
            _ => panic!("Unexpected!"),
        };
    }
}

fn encode_reg(opcode: u8, r_a: u8, r_b: u8) -> u8 {
    return (opcode << 4) | (r_a << 2) | r_b;
}

fn encode_imm(opcode: u8, r_a: u8, i: u8, nibble: Nibble) -> u8 {
    let i = match nibble {
        Nibble::Low => i & 0x0F,
        Nibble::High => (i & 0xF0) >> 4,
    };
    let i = ((i & 0b1100) << 2) | (i & 0b0011);
    return (opcode << 6) | (r_a << 2) | i;
}
