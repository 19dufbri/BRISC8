using System;

namespace BRISC8VirtualMachine
{
    public class ConsolePeripheral : IPeripheral
    {
        public byte? RunCycle()
        {
            return null;
        }

        public void DoWrite(byte addr, byte value)
        {
            Console.Write((char) value);
        }

        public byte DoRead(byte addr)
        {
            return 0;
        }
    }
}