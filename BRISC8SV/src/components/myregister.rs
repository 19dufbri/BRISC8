use marlin::verilog::prelude::*;

#[verilog(src = "./src/sv/myregister.sv", name = "myregister")]
pub struct MyRegister;
