use brisc8sv::components::register_file::RegisterFile;
use marlin::verilator::{AsDynamicVerilatedModel, VerilatorRuntime, VerilatorRuntimeOptions};
use snafu::Whatever;
use std::path::Path;

#[test]
//#[snafu::report]
fn myregister() -> Result<(), Whatever> {
    let runtime = VerilatorRuntime::new(
        Path::new("build"),
        &[
            "src/sv/register_file.sv".as_ref(),
            "src/sv/register8.sv".as_ref(),
        ],
        &[],
        [],
        VerilatorRuntimeOptions::default(),
    )
    .unwrap();

    set_value_tests(&runtime)?;
    read_value_tests(&runtime)?;
    swap_tests(&runtime)?;
    reset_tests(&runtime)?;

    Ok(())
}

fn set_value_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<RegisterFile>().unwrap();

    regs.main_bus = 0x55;
    regs.a_select = 0b00;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0x00_00_00_55);

    regs.main_bus = 0xFF;
    regs.write_select = 0b01;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0x00_00_FF_55);

    regs.main_bus = 0xEF;
    regs.write_select = 0b10;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0x00_EF_FF_55);

    regs.main_bus = 0xBE;
    regs.write_select = 0b11;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xBE_EF_FF_55);

    regs.main_bus = 0xDE;
    regs.write_select = 0b01;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xBE_EF_DE_55);

    regs.main_bus = 0xAD;
    regs.write_select = 0b00;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xBE_EF_DE_AD);

    Ok(())
}

fn read_value_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<RegisterFile>().unwrap();

    const RESULTS: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    for i in 0..4 {
        write_reg(&mut regs, i, RESULTS[i as usize]);
    }

    for a in 0..4 {
        for b in 0..4 {
            for pc_sel in 0..=1 {
                regs.a_select = a;
                regs.b_select = b;
                regs.b_address_select = pc_sel;
                do_clock(&mut regs);

                assert_eq!(regs.alu_a, RESULTS[a as usize]);
                assert_eq!(regs.alu_b, RESULTS[b as usize]);
                if pc_sel == 0 {
                    assert_eq!(regs.address, RESULTS[0b11]);
                } else {
                    assert_eq!(regs.address, RESULTS[b as usize]);
                }
            }
        }
    }

    Ok(())
}

fn swap_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<RegisterFile>().unwrap();

    const RESULTS: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    for i in 0..4 {
        // reverse for reading order
        write_reg(&mut regs, i, RESULTS[3 - i as usize]);
    }

    assert_eq!(regs.dbg_reg_state, 0xDE_AD_BE_EF);

    // Swap
    regs.swap_en = 1;
    regs.a_select = 0b00;
    regs.b_select = 0b10;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xDE_EF_BE_AD);

    // Do nothing if no swap
    regs.swap_en = 0;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xDE_EF_BE_AD);

    // Swap with self is noop
    regs.swap_en = 1;
    regs.a_select = 0b11;
    regs.b_select = 0b11;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xDE_EF_BE_AD);

    // Swap again
    regs.swap_en = 1;
    regs.a_select = 0b11;
    regs.b_select = 0b01;
    do_clock(&mut regs);
    assert_eq!(regs.dbg_reg_state, 0xBE_EF_DE_AD);

    Ok(())
}

fn reset_tests(runtime: &VerilatorRuntime) -> Result<(), Whatever> {
    let mut regs = runtime.create_model_simple::<RegisterFile>().unwrap();

    const RESULTS: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    for i in 0..4 {
        // reverse for reading order
        write_reg(&mut regs, i, RESULTS[3 - i as usize]);
    }

    assert_eq!(regs.dbg_reg_state, 0xDE_AD_BE_EF);

    // Reset async
    regs.reset = 1;
    regs.eval();
    assert_eq!(regs.dbg_reg_state, 0x00_00_00_00);

    // Entering data is noop under reset
    regs.clock = 1;
    regs.main_bus = 0xAB;
    regs.eval();
    regs.clock = 0;
    regs.eval();
    assert_eq!(regs.dbg_reg_state, 0x00_00_00_00);

    // Coming out of reset restores function
    regs.reset = 0;
    regs.eval();
    write_reg(&mut regs, 0b00, 0xAB);
    assert_eq!(regs.dbg_reg_state, 0x00_00_00_AB);

    Ok(())
}

fn write_reg(regs: &mut RegisterFile, reg: u8, data: u8) {
    regs.main_bus = data;
    regs.write_select = reg;
    do_clock(regs);
    let result = match reg {
        0b00 => regs.dbg_reg_state & 0xFF,
        0b01 => (regs.dbg_reg_state >> 8) & 0xFF,
        0b10 => (regs.dbg_reg_state >> 16) & 0xFF,
        0b11 => (regs.dbg_reg_state >> 24) & 0xFF,
        _ => panic!("Register out of range, expected [0, 3] got {reg}"),
    };
    assert_eq!(result as u8, data);
}

fn do_clock(regs: &mut RegisterFile) {
    regs.eval();
    regs.clock = 1;
    regs.eval();
    regs.clock = 0;
    regs.eval();
}
