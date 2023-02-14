namespace BRISC8VirtualMachine
{
    public class SystemPeripheral : IPeripheral
    {
        public byte? RunCycle()
        {
            return null;
        }

        public void DoWrite(byte addr, byte value)
        {
            Environment.Exit(value);
        }

        public byte DoRead(byte addr)
        {
            return 0x00;
        }
    }
}