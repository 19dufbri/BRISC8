using System;

namespace BRISC8VirtualMachine {
	public static class Program
	{
		private static void Main(string[] args) {
			if (args.Length < 1) {
				Console.Error.WriteLine("Too few arguments!");
				Console.Error.WriteLine("Usage: " + Environment.CommandLine + " <binary_file>");
				return;
			}

			var core = new VirtualMachine();

			// Read in program
			var memory = File.ReadAllBytes(args[0]);
			if (memory.Length > 0x100)
				return;
			for (byte addr = 0; addr < memory.Length; addr++)
				core.SetMemory(addr, memory[addr]);

			// Run forever 
			while (true)
				core.DoCycle();
		}
	}
}