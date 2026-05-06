use brisc8sv::components::register8::Register8;
use marlin::verilator::{AsDynamicVerilatedModel, VerilatorRuntime, VerilatorRuntimeOptions};
use snafu::Whatever;
use std::path::Path;

#[test]
//#[snafu::report]
fn register8() -> Result<(), Whatever> {
    let runtime = VerilatorRuntime::new(
        Path::new("build"),
        &["src/sv/register8.sv".as_ref()],
        &[],
        [],
        VerilatorRuntimeOptions::default(),
    )
    .unwrap();

    let mut reg = runtime.create_model_simple::<Register8>().unwrap();

    do_cycle(&mut reg, 0x00, 0, 0, 0, 0x00);
    do_cycle(&mut reg, 0x55, 0, 0, 0, 0x00);
    do_cycle(&mut reg, 0x55, 0, 1, 0, 0x00);
    do_cycle(&mut reg, 0x55, 0, 1, 1, 0x55);
    do_cycle(&mut reg, 0xAA, 0, 1, 0, 0x55);
    do_cycle(&mut reg, 0xAA, 0, 0, 1, 0x55);
    do_cycle(&mut reg, 0xAA, 0, 0, 0, 0x55);
    do_cycle(&mut reg, 0xAA, 1, 0, 0, 0x00);
    do_cycle(&mut reg, 0xAA, 1, 1, 1, 0x00);
    do_cycle(&mut reg, 0xAA, 0, 1, 1, 0xAA);

    Ok(())
}

fn do_cycle(reg: &mut Register8, input: u8, reset: u8, clock: u8, select: u8, expected: u8) {
    reg.data_in = input;
    reg.reset = reset;
    reg.clock = clock;
    reg.select = select;
    reg.eval();
    assert_eq!(reg.data_out, expected);
}
