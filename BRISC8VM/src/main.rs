use clap::Parser;

use crate::peripherals::{Console, System};

mod brisc8core;
mod peripherals;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "Binary Path")]
    prog_path: String,
}

fn main() {
    let args = Args::parse();
    let program = std::fs::read(args.prog_path).unwrap();
    
    println!("Loading Program...");
    let mut core = brisc8core::Brisc8core::new();
    for (index, byte) in program.iter().enumerate() {
        core.set_memory(index as u8, *byte);
        print!("{:x} ", byte);
    }

    core.set_peripheral(0x00, Box::new(Console {}));
    core.set_peripheral(0xFF, Box::new(System {}));

    println!("Starting BRISC8 Core...");
    loop {
        core.do_cycle();
    }
}
