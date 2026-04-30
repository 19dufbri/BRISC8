use std::{collections::VecDeque, io::{BufRead, Lines}, iter::Peekable};

use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Tokenizer<T: BufRead> {
    input: Peekable<Lines<T>>,
    literal_part: Option<VecDeque<u8>>,
    error: bool,
}

#[derive(Debug)]
pub enum Token {
    Label(String),
    Instruction(String, Vec<InstructionArg>),
    String(String),
    Byte(u8),
}

#[derive(Debug)]
pub enum InstructionArg {
    Register(u8),
    Immediate(u16),
    Label(String),
}

impl<T: BufRead> Tokenizer<T> {
    pub fn new(input: T) -> Tokenizer<T> {
        return Tokenizer {
            input: input.lines().peekable(),
            literal_part: None,
            error: false,
        };
    }
}

impl<T: BufRead> Iterator for Tokenizer<T> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        // handle deferred literals
        if let Some(literal) = &mut self.literal_part {
            if let Some(result) = literal.pop_front() {
                return Some(Ok(Token::Byte(result)));
            }
        }

        let line = self.input.next()?.ok()?;
        let mut line = line.trim_start();

        // comment
        if line.contains("--") {
            line = line.split_once("--").unwrap().0.trim();
        }

        // literal
        if line.starts_with('.') {
            let mut line = &line[1..];
            if line.starts_with('"') {
                self.literal_part = Some(parse_string(line).into());
            } else {
                if line.starts_with("0x") || line.starts_with("0X") {
                    line = &line[2..];
                }
                self.literal_part = Some(parse_hex_literal(line).into());
            }
            return self.next();
        }

        // label
        if line.ends_with(':') {
            let label = &line[..line.len()-1];
            return Some(Ok(Token::Label(label.into())));
        }

        if line.is_empty() {
            return self.next();
        }

        let (instruction, args) = line.split_once(' ')?;
        let args = args.split(',').map(|x| parse_arg(x)).collect::<Result<Vec<_>, _>>();
        if let Err(err) = args {
            self.error = true;
            return Some(Err(err));
        }
        return Some(Ok(Token::Instruction(instruction.into(), args.unwrap())));
    }
}

fn char_store(buffer: &mut Vec<u8>, char: char) -> () {
    let mut buf = [0; 4];
    char.encode_utf8(&mut buf);
    for x in 0..char.len_utf8() {
        buffer.push(buf[x]);
    }
}

fn parse_hex_literal(line: &str) -> Vec<u8> {
    let mut val = 0u8;
    let mut value = Vec::new();
    let mut next = false;
    for x in line.chars() {
        match x {
            '0' => val += 0x0,
            '1' => val += 0x1,
            '2' => val += 0x2,
            '3' => val += 0x3,
            '4' => val += 0x4,
            '5' => val += 0x5,
            '6' => val += 0x6,
            '7' => val += 0x7,
            '8' => val += 0x8,
            '9' => val += 0x9,
            'a' | 'A' => val += 0xA,
            'b' | 'B' => val += 0xB,
            'c' | 'C' => val += 0xC,
            'd' | 'D' => val += 0xD,
            'e' | 'E' => val += 0xE,
            'f' | 'F' => val += 0xF,
            _ => panic!("Unhandled character while parsing hex")
        }
        if !next {
            next = true;
            val = val << 4;
        } else {
            next = false;
            value.push(val);
            val = 0;
        }
    }
    if next {
        panic!("Hex not correctly terminated!");
    }

    return value;
}

fn parse_string(line: &str) -> Vec<u8> {
    // string
    let mut escape = false;
    let mut value = Vec::new();
    for char in line[1..].chars() {
        if escape {
            match char {
                'n' => char_store(&mut value, '\n'),
                'r' => char_store(&mut value, '\r'),
                't' => char_store(&mut value, '\t'),
                '\\' => char_store(&mut value, '\\'),
                '0' => char_store(&mut value, '\0'),
                '\'' => char_store(&mut value, '\''),
                '"' => char_store(&mut value, '"'),
                _ => panic!("Unhandled escaped char."),
            }
            escape = false;
        } else {
            if char == '\\' {
                escape = true;
                continue;
            } else if char == '"' {
                break;
            }
            char_store(&mut value, char);
        }
    }
    if escape {
        panic!("String not correctly terminated!");
    }

    return value;
}

fn parse_arg(arg_init: &str) -> Result<InstructionArg> {
    let arg = arg_init.trim();

    if arg.starts_with('#') {
        // immediate
        let arg = &arg[1..];
        if arg.starts_with("0x") || arg.starts_with("0X") {
            // hex
            let arg = &arg[2..];
            let value = u16::from_str_radix(arg, 16)?;
            return Ok(InstructionArg::Immediate(value));
        } else {
            // decimal
            let signed_value = i32::from_str_radix(arg, 10)?;
            return Ok(InstructionArg::Immediate(signed_value as u16));
        }
    } else if arg.starts_with('%') {
        // register
        let arg = &arg[1..];
        if !arg.starts_with('R') {
            return Err(anyhow!("Expected instruction register argument, got '{}'", arg_init));
        }
        let arg = &arg[1..];
        return Ok(InstructionArg::Register(u8::from_str_radix(arg, 10)?));
    } else if arg.starts_with('$') {
        // label
        let arg = &arg[1..];
        return Ok(InstructionArg::Label(arg.into()));
    } else {
        return Err(anyhow!("Expected instruction argument, got '{}'", arg_init));
    }
}