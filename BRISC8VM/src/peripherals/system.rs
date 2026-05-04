use crate::peripherals::Peripheral;

pub struct System { }

impl Peripheral for System {
    fn do_cycle(&mut self) -> () { }

    fn do_write(&mut self, _addr: u8, value: u8) -> () {
        println!();
        println!("Exited with code {value}");
        std::process::exit(value as i32);
    }

    fn do_read(&mut self, _addr: u8) -> u8 {
        return 0;
    }
}