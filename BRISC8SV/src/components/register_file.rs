use marlin::verilog::prelude::*;

#[verilog(src = "./src/sv/register_file.sv", name = "register_file")]
pub struct RegisterFile;
