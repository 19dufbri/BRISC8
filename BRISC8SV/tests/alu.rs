use brisc8sv::components::alu::Alu;
use marlin::verilator::{AsDynamicVerilatedModel, VerilatorRuntime, VerilatorRuntimeOptions};
use snafu::Whatever;
use std::path::Path;

#[test]
#[snafu::report]
fn myregister() -> Result<(), Whatever> {
    let runtime = VerilatorRuntime::new(
        Path::new("build"),
        &["src/sv/alu.sv".as_ref()],
        &[],
        [],
        VerilatorRuntimeOptions::default(),
    )
    .unwrap();

    load_imm_tests(&runtime)?;
    add_tests(&runtime)?;
    nand_tests(&runtime)?;

    Ok(())
}

fn load_imm_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<Alu>().unwrap();

    regs.load_immediate = 1;
    regs.immediate_hilo = 0;

    // Sign extend
    regs.a_input = 0x00;
    regs.immediate = 0x0F;
    regs.eval();
    assert_eq!(regs.data_ouput, 0xFF);
    regs.a_input = 0x00;
    regs.immediate = 0x07;
    regs.eval();
    assert_eq!(regs.data_ouput, 0x07);

    // Overwrite top
    regs.a_input = 0xAC;
    regs.immediate = 0x0F;
    regs.eval();
    assert_eq!(regs.data_ouput, 0xFF);
    regs.a_input = 0xAC;
    regs.immediate = 0x06;
    regs.eval();
    assert_eq!(regs.data_ouput, 0x06);
    regs.a_input = 0xAC;
    regs.immediate = 0x0B;
    regs.eval();
    assert_eq!(regs.data_ouput, 0xFB);

    regs.load_immediate = 1;
    regs.immediate_hilo = 1;

    regs.a_input = 0x00;
    regs.immediate = 0x07;
    regs.eval();
    assert_eq!(regs.data_ouput, 0x70);
    regs.a_input = 0x45;
    regs.immediate = 0x07;
    regs.eval();
    assert_eq!(regs.data_ouput, 0x75);

    Ok(())
}

fn add_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<Alu>().unwrap();

    regs.operation_type = 0;

    // standard
    regs.a_input = 25;
    regs.b_input = 25;
    regs.eval();
    assert_eq!(regs.data_ouput, 50);

    // negatives
    regs.a_input = -1i8 as u8;
    regs.b_input = 65;
    regs.eval();
    assert_eq!(regs.data_ouput, 64);

    // overflow
    regs.a_input = 200;
    regs.b_input = 60;
    regs.eval();
    assert_eq!(regs.data_ouput, 4);

    Ok(())
}

fn nand_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<Alu>().unwrap();

    regs.operation_type = 1;

    // standard
    regs.a_input = 0b0000_0000;
    regs.b_input = 0b0000_0000;
    regs.eval();
    assert_eq!(regs.data_ouput, 0b1111_1111);
    regs.a_input = 0b1010_1010;
    regs.b_input = 0b0101_0101;
    regs.eval();
    assert_eq!(regs.data_ouput, 0b1111_1111);
    regs.a_input = 0b1101_0010;
    regs.b_input = 0b1011_0100;
    regs.eval();
    assert_eq!(regs.data_ouput, 0b0110_1111);

    Ok(())
}
