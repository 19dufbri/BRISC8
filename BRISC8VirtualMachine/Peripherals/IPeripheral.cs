namespace BRISC8VirtualMachine.Peripherals
{
  public interface IPeripheral
  {
    byte? RunCycle();
    void DoWrite(byte addr, byte value);
    byte DoRead(byte addr);
  }
}