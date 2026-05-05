use brisc8sv::components::myregister::MyRegister;
use marlin::verilator::{AsDynamicVerilatedModel, VerilatorRuntime, VerilatorRuntimeOptions};
use snafu::Whatever;
use std::path::Path;

#[test]
//#[snafu::report]
fn myregister() -> Result<(), Whatever> {
    let runtime = VerilatorRuntime::new(
        Path::new("build"),
        &["src/sv/myregister.sv".as_ref(), "src/sv/latch.sv".as_ref()],
        &[],
        [],
        VerilatorRuntimeOptions::default(),
    )
    .unwrap();

    let mut reg = runtime.create_model_simple::<MyRegister>().unwrap();

    assert_eq!(reg.data_out, 0);

    Ok(())
}
