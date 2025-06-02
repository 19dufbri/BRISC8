using BRISC8VirtualMachine.Peripherals;

namespace BRISC8VirtualMachine
{
  public class VirtualMachine
  {
    private readonly byte[] registers = new byte[0x4];
    private readonly byte[] memory = new byte[0x10000];

    private ushort ProgramCounter;
    private ushort StackPointer;

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
      memory[address] = data;
    }

    private void DoPeripherals()
    {
      for (var i = 0; i < _peripherals.Length; i++)
      {
        var t = _peripherals[i];
        var interrupt = t?.RunCycle();

        if (!interrupt.HasValue)
          continue;

        registers[0x0] = (byte)i;
        registers[0x1] = interrupt.Value;
        break;
      }
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
      var opcode = memory[registers[ProgramCounter]++];

      // Register Selectors
      var rA = (byte)((opcode >> 2) & 0x03);
      var rB = (byte)(opcode & 0x03);

      // Opcode decode
      switch (opcode >> 4)
      {
        case 0b0000:
          /*
					 * LIL #i
					 * Load low 4 bits of instruction into low four bits of R0
					 */
          registers[0] = (byte)((registers[0] & 0xF0) | (opcode & 0x0F));
          break;
        case 0b0001:
          /*
					 * LIH #i
					 * Load low 4 bits of instruction into high four bits of R0
					 */
          registers[0] = (byte)(((opcode & 0x0F) << 4) | (registers[0] & 0x0F));
          break;
        case 0b0010:
          /*
           * PSH Ra
           */
          if ((rB & 0b10) == 0b10)
          {
            // Special case, PC or SP
            if ((rB & 0b01) == 0b00)
            {
              memory[StackPointer++] = (byte)(ProgramCounter >> 8);
              memory[StackPointer++] = (byte)(ProgramCounter & 0x0F);
            }
            else
            {
              var spValue = StackPointer;
              memory[StackPointer++] = (byte)(spValue >> 8);
              memory[StackPointer++] = (byte)(spValue & 0x0F);
            }
          }
          else
          {
            memory[StackPointer++] = registers[rA];
          }
          break;
        case 0b0011:
          /*
           * POP rA
           */
          if ((rB & 0b10) == 0b10)
          {
            // Special case, PC or SP
            if ((rB & 0b01) == 0b00)
            {
              ushort value = 0;
              value |= memory[--StackPointer];
              value |= (ushort)(memory[--StackPointer] << 8);
              ProgramCounter = value;
            }
            else
            {
              ushort value = 0;
              value |= memory[--StackPointer];
              value |= (ushort)(memory[--StackPointer] << 8);
              StackPointer = value;
            }
          }
          else
          {
            registers[rA] = memory[--StackPointer];
          }
          break;
        case 0b0100:
          /*
           * MOVx PC, RaRb
           */
          registers[rA] = (byte)(ProgramCounter >> 8);
          registers[rB] = (byte)(ProgramCounter & 0xF);
          break;
        case 0b0101:
          /*
           * MOV SP, RaRb
           */
          registers[rA] = (byte)(StackPointer >> 8);
          registers[rB] = (byte)(StackPointer & 0xF);
          break;
        case 0b0110:
          /*
           * MOV RaRb, PC
           */
          ProgramCounter = 0;
          ProgramCounter |= (ushort)(registers[rA] << 8);
          ProgramCounter |= registers[rB];
          break;
        case 0b0111:
          /*
           * MOV RaRb, SP
           */
          StackPointer = 0;
          StackPointer |= (ushort)(registers[rA] << 8);
          StackPointer |= registers[rB];
          break;
        case 0b1000:
          /*
					 * ADD Ra, Rb
					 * Add Ra into Rb
					 */
          registers[rB] = (byte)(registers[rA] + registers[rB]);
          break;
        case 0b1001:
          /*
					 * NOR Ra, Rb
					 * NOR Ra into Rb
					 */
          registers[rB] = (byte)~(registers[rA] | registers[rB]);
          break;
        case 0b1010:
          /*
					 * IOR Ra, Rb
					 * Read IO location A into Rb
					 */
          registers[rB] = (_peripherals[registers[rA]]?.DoRead(registers[rA])).GetValueOrDefault();
          break;
        case 0b1011:
          /*
					 * IOW Ra, Rb
					 * Write Rb into IO location A
					 */
          _peripherals[registers[rA]]?.DoWrite(registers[rA], registers[rB]);
          break;
        case 0b1100:
          /*
					 * LOA Ra, Rb
					 * Load memory address Ra into Rb
					 */
          registers[rB] = memory[registers[rA]];
          break;
        case 0b1101:
          /*
					 * STO Ra, Rb
					 * Store Rb at Ra in memory
					 */
          memory[registers[rA]] = registers[rB];
          break;
        case 0b1110:
          /*
					 * SKL Ra, Rb
					 * Skip the next instruction if Ra < Rb
					 */
          if ((sbyte)registers[rA] < (sbyte)registers[rB])
          {
            registers[ProgramCounter]++;
          }
          break;
        case 0b1111:
          /*
           * MOV Ra, Rb
           */
          registers[rB] = registers[rA];
          break;
        default:
          throw new NotImplementedException();
      }
    }
  }
}