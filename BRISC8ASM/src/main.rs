use std::io::{BufWriter, Write};
use std::{fs::File, io::BufReader};

use clap::Parser;

use crate::lexer::Lexer;
use crate::generator::Generator;

mod lexer;
mod generator;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "Assembly File")]
    input_file: String,
    #[arg(value_name = "Binary File")]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.input_file).unwrap();
    let reader = BufReader::new(file);
    let tokenizer = Lexer::new(reader);
    let tokens = tokenizer.collect::<Result<Vec<_>, _>>();
    if let Err(err) = tokens {
        dbg!(err);
        panic!("At the disco!");
    }

    let mut generator = Generator::new();
    let (result, size) = generator.assemble(tokens.unwrap()).unwrap();

    let file = File::create(args.output_file).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(&result).unwrap();

    println!("Done! Program is {size} bytes, {:.02}% of memory", size as f64 / 2.56f64);
}
