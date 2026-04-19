pub trait Peripheral {
    /*
    void RunCycle();
    void DoWrite(byte addr, byte value);
    byte DoRead(byte addr);
     */
    fn do_cycle(&mut self) -> ();
    fn do_write(&mut self, addr: u8, value: u8) -> ();
    fn do_read(&mut self, addr: u8) -> u8;
}
