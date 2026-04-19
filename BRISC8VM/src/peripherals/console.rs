use crate::peripherals::Peripheral;

pub struct Console { }

impl Peripheral for Console {
    fn do_cycle(&mut self) -> () { }

    fn do_write(&mut self, _addr: u8, value: u8) -> () {
        print!("{:}", char::from(value))
    }

    fn do_read(&mut self, _addr: u8) -> u8 {
        return 0;
    }
}
