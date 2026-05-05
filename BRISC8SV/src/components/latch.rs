// file: src/lib.rs
use marlin::verilog::prelude::*;

#[verilog(src = "src/sv/latch.sv", name = "latch")]
pub struct Latch;