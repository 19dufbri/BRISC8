namespace BRISC8VirtualMachine
{
  public interface IPeripheral
  {
    void RunCycle();
    void DoWrite(byte addr, byte value);
    byte DoRead(byte addr);
  }
}