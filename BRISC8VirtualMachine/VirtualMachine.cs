namespace BRISC8VirtualMachine
{
  public class VirtualMachine
  {
    private readonly byte[] _registers = new byte[0x04];
    private readonly byte[] _memory = new byte[0x100];

    private const int Pc = 0x3;

    private readonly IPeripheral?[] _peripherals;

    public VirtualMachine()
    {
      _peripherals = new IPeripheral?[0x100];
      _peripherals[0x00] = new ConsolePeripheral();
      _peripherals[0x01] = _peripherals[0x02] = new TimerPeripheral();
      _peripherals[0xFF] = new SystemPeripheral();
    }

    public void SetMemory(byte address, byte data)
    {
      _memory[address] = data;
    }

    private void DoPeripherals()
    {
      for (var i = 0; i < _peripherals.Length; i++)
        _peripherals[i]?.RunCycle();
    }

    public void DoCycle()
    {
      // Do peripherals such as timer, input, and output
      DoPeripherals();

      // Run one instruction
      DoOpcode();
    }

    private void DoOpcode()
    {
      // Opcode to run
      var opcode = _memory[_registers[Pc]++];

      // Register Selectors
      var rA = (byte)((opcode >> 2) & 0x03);
      var rB = (byte)(opcode & 0x03);
      var immediate = (byte)(((opcode & 0x30) >> 2) | (opcode & 0x03));

      // Opcode decode
      switch (opcode >> 4)
      {
        case 0b0000:
        case 0b0001:
        case 0b0010:
        case 0b0011:
          /*
					 * LIL #i, RX
					 * Load low 4 bits of instruction into low four bits of Rx
					 */
          _registers[rA] = (byte)((_registers[rA] & 0xF0) | immediate);
          break;
        case 0b0100:
        case 0b0101:
        case 0b0110:
        case 0b0111:
          /*
					 * LIH #i, Rx
					 * Load low 4 bits of instruction into high four bits of Rx
					 */
          _registers[rA] = (byte)((immediate << 4) | (_registers[rA] & 0x0F));
          break;
        case 0b1000:
          /*
					 * ADD Ra, Rb
					 * Add Ra into Rb
					 */
          _registers[rA] = (byte)(_registers[rA] + _registers[rB]);
          break;
        case 0b1001:
          /*
					 * NAND Ra, Rb
					 * NAND Ra into Rb
					 */
          _registers[rA] = (byte)~(_registers[rA] & _registers[rB]);
          break;
        case 0b1010:
          /*
					 * IOR Ra, Rb
					 * Read IO location A into Rb
					 */
          _registers[rA] = (_peripherals[_registers[rB]]?.DoRead(_registers[rB])).GetValueOrDefault();
          break;
        case 0b1011:
          /*
					 * IOW Ra, Rb
					 * Write Rb into IO location A
					 */
          _peripherals[_registers[rB]]?.DoWrite(_registers[rB], _registers[rA]);
          break;
        case 0b1100:
          /*
					 * LOA Ra, Rb
					 * Load memory address Ra into Rb
					 */
          _registers[rA] = _memory[_registers[rB]];
          break;
        case 0b1101:
          /*
					 * STO Ra, Rb
					 * Store Rb at Ra in memory
					 */
          _memory[_registers[rB]] = _registers[rA];
          break;
        case 0b1110:
          /*
					 * SKL Ra, Rb
					 * Skip the next instruction if Ra < Rb
					 */
          if ((sbyte)_registers[rA] < (sbyte)_registers[rB])
          {
            _registers[Pc]++;
          }
          break;
        case 0b1111:
          throw new NotImplementedException();
      }
    }
  }
}