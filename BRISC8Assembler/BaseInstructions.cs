using Microsoft.Win32;

namespace BRISC8Assembler
{
  public static class BaseInstructions
  {
    public static byte LoadImmediateLow(byte value)
      => (byte)((value & 0xF) | 0x00);

    public static byte LoadImmediateHigh(byte value)
      => (byte)((value & 0xF) | 0x10);

    public static byte PushRegister(byte register)
      => RegisterInstruction(0b0010, register);

    public static byte PushProgramCounter()
      => RegisterInstruction(0b0010, 0b00, 0b10);

    public static byte PushStackPointer()
      => RegisterInstruction(0b0010, 0b00, 0b11);

    public static byte PopRegister(byte register)
      => RegisterInstruction(0b0011, register);

    public static byte PopProgramCounter()
      => RegisterInstruction(0b0011, 0b00, 0b10);

    public static byte PopStackPointer()
      => RegisterInstruction(0b0011, 0b00, 0b11);

    public static byte MoveProgramCounterToRegisterPair(byte highRegister, byte lowRegister)
      => RegisterInstruction(0b0100, highRegister, lowRegister);

    public static byte MoveStackPointerToRegisterPair(byte highRegister, byte lowRegister)
      => RegisterInstruction(0b0101, highRegister, lowRegister);

    public static byte MoveRegisterPairToProgramCounter(byte highRegister, byte lowRegister)
      => RegisterInstruction(0b0110, highRegister, lowRegister);

    public static byte MoveRegisterPairToStackPointer(byte highRegister, byte lowRegister)
      => RegisterInstruction(0b0111, highRegister, lowRegister);

    /*
| Bit Pattern |   Mnemonic   | Description                             |
|-------------|--------------|-----------------------------------------|
| 0b1100 rrrr | LOA rA, rB	 | Load memory[rA] into rB                 |
| 0b1101 rrrr | STO rA, rB	 | Store rB into memory[rA]                |
| 0b1110 rrrr | SKL rA, rB	 | Skip the next instruction if rA < rB    |
| 0b1111 rrrr | MOV rA, rB   | Copy rA to rB                           |
    */

    public static byte Add(byte sourceRegister, byte destinationRegister)
      => RegisterInstruction(0b1000, sourceRegister, destinationRegister);

    public static byte Nor(byte sourceRegister, byte destinationRegister)
      => RegisterInstruction(0b1001, sourceRegister, destinationRegister);

    public static byte IORead(byte ioAddress, byte destinationRegister)
      => RegisterInstruction(0b1010, ioAddress, destinationRegister);

    public static byte IOWrite(byte ioAddress, byte sourceRegister)
      => RegisterInstruction(0b1011, ioAddress, sourceRegister);

    static byte RegisterInstruction(byte opcode, byte rA, byte rB = 0)
      => (byte)((opcode << 4) | ((rA & 0x3) << 2) | (rB & 0x3));
  }
}