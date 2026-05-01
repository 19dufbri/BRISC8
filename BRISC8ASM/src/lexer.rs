use std::{collections::VecDeque, io::{BufRead, Lines}, iter::Peekable};

use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct Lexer<T: BufRead> {
    input: Peekable<Lines<T>>,
    literal_part: Option<VecDeque<u8>>,
    error: bool,
}

#[derive(Debug)]
pub enum Token {
    Label(String),
    Instruction(String, Vec<InstructionArg>),
    Byte(u8),
    Directive(String),
}

#[derive(Debug)]
pub enum InstructionArg {
    Register(u8),
    Immediate(u8),
    Label(String),
}

impl<T: BufRead> Lexer<T> {
    pub fn new(input: T) -> Lexer<T> {
        return Lexer {
            input: input.lines().peekable(),
            literal_part: None,
            error: false,
        };
    }
}

impl<T: BufRead> Iterator for Lexer<T> {
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

        // directive
        if line.starts_with('#') {
            return Some(Ok(Token::Directive(line[1..].into())));
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

trait StoreChar {
    fn push_char(&mut self, value: char);
}

impl StoreChar for Vec<u8> {
    fn push_char(&mut self, value: char) {
        let mut buf = [0; 4];
        value.encode_utf8(&mut buf);
        for x in 0..value.len_utf8() {
            self.push(buf[x]);
        }
    }
}

fn parse_hex_literal(mut line: &str) -> Vec<u8> {
    let mut value = Vec::new();
    while line.len() >= 2 {
        let result = line.split_at(2);
        line = result.1;
        value.push(u8::from_str_radix(result.0, 16).unwrap());
    }

    return value;
}

fn parse_string(line: &str) -> Vec<u8> {
    // string
    let mut escape = false;
    let mut value: Vec<u8> = Vec::new();
    for char in line[1..].chars() {
        if escape {
            match char {
                'n' => value.push_char('\n'),
                'r' => value.push_char('\r'),
                't' => value.push_char('\t'),
                '\\' => value.push_char('\\'),
                '0' => value.push_char('\0'),
                '\'' => value.push_char('\''),
                '"' => value.push_char('"'),
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
            value.push_char(char);
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
            let value = u8::from_str_radix(arg, 16)?;
            return Ok(InstructionArg::Immediate(value as u8));
        } else {
            // decimal
            let signed_value = i16::from_str_radix(arg, 10)?;
            return Ok(InstructionArg::Immediate(signed_value as u8));
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