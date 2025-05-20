namespace BRISC8Assembler
{
  public static class BaseInstructions
  {
    public static List<byte> bListOf(params byte[] bytes)
      => bytes.ToList();

    public static List<byte> InstructionLil(byte rX, byte i)
      => bListOf((byte)((rX << 4) | (i & 0x0F)));

    public static List<byte> InstructionLih(byte rX, byte i)
      => bListOf((byte)(0x40 | (rX << 4) | ((i & 0xF0) >> 4)));

    public static List<byte> InstructionAdd(byte rA, byte rB)
      => bListOf((byte)(0x80 | (rA << 2) | rB));

    public static List<byte> InstructionNand(byte rA, byte rB)
      => bListOf((byte)(0x90 | (rA << 2) | rB));

    public static List<byte> InstructionIor(byte rA, byte rB)
      => bListOf((byte)(0xA0 | (rA << 2) | rB));

    public static List<byte> InstructionIow(byte rA, byte rB)
      => bListOf((byte)(0xB0 | (rA << 2) | rB));

    public static List<byte> InstructionLoa(byte rA, byte rB)
      => bListOf((byte)(0xC0 | (rA << 2) | rB));

    public static List<byte> InstructionSto(byte rA, byte rB)
      => bListOf((byte)(0xD0 | (rA << 2) | rB));

    public static List<byte> InstructionSkl(byte rA, byte rB)
      => bListOf((byte)(0xE0 | (rA << 2) | rB));

    public static List<byte> InstructionInt(byte ab)
      => bListOf((byte)(0xF0 | ab));

    public static List<byte> PseudoLir(byte rX, byte i)
      => InstructionLil(rX, i).Add(InstructionLih(rX, i));

    public static List<byte> PseudoSub(byte rA, byte rB)
      => PseudoNot(rB)
        .Add(PseudoInc(rB))
        .Add(InstructionAdd(rA, rB));

    public static List<byte> PseudoNot(byte rA)
      => PseudoLir(0x2, 0xFF)
        .Add(InstructionNand(0x2, rA));

    public static List<byte> PseudoAddI(byte rA, byte i)
      => PseudoLir(0x2, i)
        .Add(InstructionAdd(0x2, rA));

    public static List<byte> PseudoInc(byte rA)
      => PseudoAddI(rA, 0x01);

    public static List<byte> PseudoDec(byte rA)
      => PseudoAddI(rA, 0xFF);

    public static List<byte> PseudoPush(byte spA)
      => PseudoLir(0x1, 0x0)
        .Add(InstructionLoa(0x1, 0x1))
        .Add(InstructionSto(0x1, 0x0))
        .Add(PseudoInc(0x1))
        .Add(PseudoLir(0x2, spA))
        .Add(InstructionSto(0x2, 0x1));

    public static List<byte> PseudoPop(byte spA)
      => PseudoLir(0x1, 0x0)
        .Add(InstructionLoa(0x1, 0x1))
        .Add(PseudoDec(0x1))
        .Add(InstructionLoa(0x1, 0x0))
        .Add(PseudoLir(0x2, spA))
        .Add(InstructionSto(0x2, 0x1));

    public static List<byte> Add(this List<byte> a, IEnumerable<byte> b)
    {
      a.AddRange(b);
      return a;
    }
  }
}