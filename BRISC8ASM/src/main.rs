use std::{fs::File, io::BufReader};

use clap::Parser;

use crate::tokenizer::Tokenizer;

mod tokenizer;

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
    let tokenizer = Tokenizer::new(reader);

    tokenizer.for_each(|x| {
        match x {
            Ok(token) => println!("{:?}", token),
            Err(err) => println!("ERROR: {}", err),
        }
    });

    println!("Done!");
}
