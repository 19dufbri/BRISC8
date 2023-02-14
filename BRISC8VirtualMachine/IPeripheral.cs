namespace BRISC8VirtualMachine
{
    public interface IPeripheral
    {
        byte? RunCycle();
        void DoWrite(byte addr, byte value);
        byte DoRead(byte addr);
    }
}