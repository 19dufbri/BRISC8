namespace BRISC8VirtualMachine
{
    public class VirtualMachine
    {
	    private enum MachineMode
	    {
		    Machine,
		    User
	    }
	    
	    private readonly byte[] _regs = new byte[0x08];
        private readonly byte[] _memory = new byte[0x100];
        private MachineMode _mode = MachineMode.Machine;
        
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
	        {
		        var t = _peripherals[i];
		        var interrupt = t?.RunCycle();
		        
		        if (!interrupt.HasValue)
			        continue;
		        
		        DoUndoInt();
		        _regs[0x0] = (byte)i;
		        _regs[0x1] = interrupt.Value;
		        break;
	        }
        }

        public void DoCycle() {
	        // Do peripherals such as timer, input, and output
			DoPeripherals();

			// Run one instruction
			DoOpcode();
		}

        private void DoUndoInt()
        {
	        // Swap register
	        for (var i = 0; i < 0x4; i++)
		        (_regs[i], _regs[i + 0x4]) = (_regs[i + 0x4], _regs[i]);
		    // Switch modes
		    _mode = _mode == MachineMode.Machine ? MachineMode.User : MachineMode.Machine;
        }

		private void DoOpcode() {
			// Opcode to run
			var opcode = _memory[_regs[Pc]++];

			// Register Selectors
			var rX = (byte) ((opcode >> 4) & 0x03);
			var rA = (byte) ((opcode >> 2) & 0x03);
			var rB = (byte) (opcode & 0x03);
			
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
					_regs[rX] = (byte) ((_regs[rX] & 0xF0) | (opcode & 0x0F));
					break;
				case 0b0100:
				case 0b0101:
				case 0b0110:
				case 0b0111:
					/*
					 * LIH #i, Rx
					 * Load low 4 bits of instruction into high four bits of Rx
					 */
					_regs[rX] = (byte) (((opcode & 0x0F) << 4) | (_regs[rX] & 0x0F));
					break;
				case 0b1000:
					/*
					 * ADD Ra, Rb
					 * Add Ra into Rb
					 */
					_regs[rB] = (byte) (_regs[rA] + _regs[rB]);
					break;
				case 0b1001:
					/*
					 * NAND Ra, Rb
					 * NAND Ra into Rb
					 */
					_regs[rB] = (byte) ~(_regs[rA] & _regs[rB]);
					break;
				case 0b1010:
					/*
					 * IOR Ra, Rb
					 * Read IO location A into Rb
					 */
					_regs[rB] = (_peripherals[_regs[rA]]?.DoRead(_regs[rA])).GetValueOrDefault();
					break;
				case 0b1011:
					/*
					 * IOW Ra, Rb
					 * Write Rb into IO location A
					 */
					_peripherals[_regs[rA]]?.DoWrite(_regs[rA], _regs[rB]);
					break;
				case 0b1100:
					/*
					 * LOA Ra, Rb
					 * Load memory address Ra into Rb
					 */
					_regs[rB] = _memory[_regs[rA]];
					break;
				case 0b1101:
					/*
					 * STO Ra, Rb
					 * Store Rb at Ra in memory
					 */
					_memory[_regs[rA]] = _regs[rB];
					break;
				case 0b1110:
					/*
					 * SKL Ra, Rb
					 * Skip the next instruction if Ra < Rb
					 */
					if ((sbyte) _regs[rA] < (sbyte)_regs[rB])
					{
						_regs[Pc]++;
					}
					break;
				case 0b1111:
					/*
					 * INT #i
					 * Call a system interrupt number AB
					 */
					DoUndoInt();
					_regs[0x0] = 0xFF;
					_regs[0x1] = (byte)(rA << 2 | rB);
					break;
			}
		}
    }
}