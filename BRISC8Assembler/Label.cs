namespace BRISC8Assembler
{
    public class Label
    {
        public Label(string name, byte addr, byte mask, byte shift)
        {
            Name = name;
            Addr = addr;
            Mask = mask;
            Shift = shift;
        }

        public string Name { get; }
        public byte Addr { get; }
        public byte Mask { get; }
        public byte Shift { get; }
    }
}