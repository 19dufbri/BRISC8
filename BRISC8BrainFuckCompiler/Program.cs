// See https://aka.ms/new-console-template for more information

using BRISC8Assembler;
using BaseInstructions = BRISC8Assembler.BaseInstructions;

namespace BRISC8BrainFuckCompiler
{
    public static class Program
    {
        private const int Data = 0x0;
        private const int Pointer = 0x1;
        private const int Scratch = 0x2;
        private const int Pc = 0x3;
        
        private static void Main(string[] args)
        {
            var input = File.ReadAllText(args[0]);
            var output = new List<byte>();

            var opens = new Stack<byte>();

            output.Add(BaseInstructions.PseudoLir(Pointer, 0xF0));

            // TODO: Optimization
            foreach (var c in input)
            {
                switch (c)
                {
                    case '+':
                        output.Add(BaseInstructions.InstructionLoa(Pointer, Data))
                            .Add(BaseInstructions.PseudoInc(Data))
                            .Add(BaseInstructions.InstructionSto(Pointer, Data));
                        break;
                    case '-':
                        output.Add(BaseInstructions.InstructionLoa(Pointer, Data))
                            .Add(BaseInstructions.PseudoDec(Data))
                            .Add(BaseInstructions.InstructionSto(Pointer, Data));
                        break;
                    case '>':
                        output.Add(BaseInstructions.PseudoInc(Pointer));
                        break;
                    case '<':
                        output.Add(BaseInstructions.PseudoDec(Pointer));
                        break;
                    case '.':
                        output.Add(BaseInstructions.PseudoLir(Scratch, 0))
                            .Add(BaseInstructions.InstructionLoa(Pointer, Data))
                            .Add(BaseInstructions.InstructionIow(Scratch, Data));
                        break;
                    case ',':
                        output.Add(BaseInstructions.PseudoLir(Scratch, 0))
                            .Add(BaseInstructions.InstructionIor(Scratch, Data))
                            .Add(BaseInstructions.InstructionSto(Pointer, Data));
                        break;
                    case '[':
                        opens.Push((byte)output.Count);
                        output.Add(BaseInstructions.PseudoLir(Scratch, 0)) // End address
                            .Add(BaseInstructions.InstructionAdd(Scratch, Pc))
                            .Add(BaseInstructions.PseudoLir(Pointer, 0xFF))
                            .Add(BaseInstructions.InstructionLoa(Pointer, Pointer));
                        break;
                    case ']':
                        var start = opens.Pop();
                        var end = output.Count;
                        output.Add(BaseInstructions.InstructionLoa(Pointer, Data))
                            .Add(BaseInstructions.PseudoLir(Scratch, 0xFF))
                            .Add(BaseInstructions.InstructionSto(Scratch, Pointer))
                            .Add(BaseInstructions.PseudoLir(Pointer, 1))
                            .Add(BaseInstructions.PseudoLir(Scratch, (byte)(start - output.Count - 1))) // Start address
                            .Add(BaseInstructions.InstructionSkl(Data, Pointer))
                            .Add(BaseInstructions.InstructionAdd(Scratch, Pc))
                            .Add(BaseInstructions.PseudoLir(Pointer, 0xFF))
                            .Add(BaseInstructions.PseudoLir(Scratch, (byte)(start - output.Count - 1))) // Start address
                            .Add(BaseInstructions.InstructionSkl(Pointer, Data))
                            .Add(BaseInstructions.InstructionAdd(Scratch, Pc))
                            .Add(BaseInstructions.PseudoLir(Pointer, 0xFF))
                            .Add(BaseInstructions.InstructionLoa(Pointer, Pointer));
                        output[start] = BaseInstructions.InstructionLil(Scratch, (byte)(end - start - 2))[0];
                        output[start+1] = BaseInstructions.InstructionLih(Scratch, (byte)(end - start - 2))[0];
                        break;
                }
            }

            output.Add(BaseInstructions.PseudoLir(Scratch, 0xFF));
            output.Add(BaseInstructions.InstructionIow(Scratch, Data));

            output[0] = BaseInstructions.InstructionLil(Pointer, (byte)output.Count)[0];
            output[1] = BaseInstructions.InstructionLih(Pointer, (byte)output.Count)[0];
            
            File.WriteAllBytes(args[1], output.ToArray());
            Console.WriteLine("Finished Compilation!");
        }
    }
}