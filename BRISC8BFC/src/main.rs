use std::{fs::File, io::Write};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "BF Input File")]
    input_file: String,
    #[arg(value_name = "Assembly Output File")]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(args.input_file).unwrap();

    // const data: u8 = 0x00;
    // const pointer: u8 = 0x01;
    // const scratch: u8 = 0x02;
    // const pc: u8 = 0x03;

    let mut result: Vec<String> = Vec::new();
    let mut loop_index = 0;
    let mut loop_stack: Vec<String> = Vec::new();

    // BF data after end of program
    result.push("LIR %R1, $ProgramEnd".into());
    for instruction in input.chars() {
        match instruction {
            '+' => {
                // Increment register
                result.push("LOA %R0, %R1".into());
                result.push("LIL %R2, #1".into());
                result.push("ADD %R0, %R2".into());
                result.push("STO %R1, %R0".into());
            },
            '-' => {
                // Decrement register
                result.push("LOA %R0, %R1".into());
                result.push("LIL %R2, #-1".into());
                result.push("ADD %R0, %R2".into());
                result.push("STO %R1, %R0".into());
            },
            '<' => {
                // Move cell right
                result.push("LIL %R2, #-1".into());
                result.push("ADD %R1, %R2".into());
            },
            '>' => {
                // Move cell left
                result.push("LIL %R2, #1".into());
                result.push("ADD %R1, %R2".into());
            },
            '.' => {
                // Output character
                result.push("LOA %R0, %R1".into());
                result.push("LIL %R2, #0".into());
                result.push("IOW %R2, %R0".into());
            },
            ',' => {
                // Input character
                result.push("LIL %R2, #0".into());
                result.push("IOR %R0, %R2".into());
                result.push("STO %R1, %R0".into());
            },
            '[' => {
                // Begin loop
                let loop_frame = format!("LoopNo_{loop_index}");
                loop_index += 1;
                loop_stack.push(loop_frame.clone());
                result.push(format!("JMP %R2, ${loop_frame}End"));
                result.push(format!("{loop_frame}Start:"));
                result.push("LIL %R2, #-1".into());
                result.push("LOA %R1, %R2".into());
            },
            ']' => {
                // Close loop
                let loop_frame = loop_stack.pop().unwrap();
                result.push(format!("{loop_frame}End:"));
                result.push("LOA %R0, %R1".into());     // Load our data
                result.push("LIL %R2, #-1".into());   // Load our pointer storage address
                result.push("STO %R2, %R1".into());     // Store pointer at that address

                result.push(format!("LIR %R1, ${loop_frame}Start"));    // Load start address into R1

                result.push("LIL %R2, #1".into());      // Number to compare
                result.push("SKL %R0, %R2".into());     // if data < 1
                result.push("JMP %R1".into());          // if data >= 1

                result.push("LIL %R2, #-1".into());
                result.push("SKL %R2, %R0".into());     // if data > -1
                result.push("JMP %R1".into());          // if data <= -1

                result.push("LIL %R2, #-1".into());
                result.push("LOA %R1, %R2".into());
            },
            _ => {}, // Non-bf chars are ignored
        }
    }
    // Closing sequence, exit with data at pointer
    result.push("LIR %R2, #-1".into());
    result.push("LOA %R0, %R1".into());
    result.push("IOW %R2, %R0".into());
    result.push("ProgramEnd:".into());
    result.push(".0x00".into());

    let mut output = File::create(args.output_file).unwrap();
    let result: Vec<u8> = result.join("\n").bytes().collect();
    output.write_all(&result).unwrap();

    println!("Done Compiling!");
}
