use brisc8sv::components::latch::Latch;
use marlin::verilator::{AsDynamicVerilatedModel, VerilatorRuntime, VerilatorRuntimeOptions};
use snafu::Whatever;
use std::path::Path;

#[test]
//#[snafu::report]
fn latches() -> Result<(), Whatever> {
    let runtime = VerilatorRuntime::new(
        Path::new("build"),
        &["src/sv/latch.sv".as_ref()],
        &[],
        [],
        VerilatorRuntimeOptions::default(),
    ).unwrap();

    let mut latch = runtime.create_model_simple::<Latch>().unwrap();

    run_step(&mut latch, 0, 0, 0, 0);
    run_step(&mut latch, 0, 0, 1, 0);
    run_step(&mut latch, 1, 0, 1, 0);
    run_step(&mut latch, 1, 1, 1, 0);
    run_step(&mut latch, 0, 1, 0, 0);
    run_step(&mut latch, 1, 1, 0, 1);
    run_step(&mut latch, 1, 0, 0, 1);
    run_step(&mut latch, 0, 0, 0, 1);
    run_step(&mut latch, 0, 0, 1, 0);

    Ok(())
}

fn run_step(latch: &mut Latch, data: u8, clock: u8, clear: u8, expected: u8) {
    latch.d = data;
    latch.clock = clock;
    latch.clear = clear;
    latch.eval();
    let result: u8 = latch.q;
    println!("D {data} Clock {clock} Clear {clear} Expected {expected} Got {result}");
    assert_eq!(result, expected);
}