using System.Globalization;

namespace BRISC8Assembler
{
    public class Generator
    {
        private string _currentFile;
        private readonly Tokenizer _tokenizer;

        public Generator(Tokenizer tokenizer, string currentFile)
        {
            _tokenizer = tokenizer;
            _currentFile = currentFile;
        }

        public List<Label> LabelDef { get; } = new();
        public List<Label> LabelRef { get; } = new();
        public List<byte> Output { get; } = new();

        public void GenerateAll()
        {
            while (!ProcessInstruction())
            {
            }
        }

        // Generate the instruction for each line of the input
        private bool ProcessInstruction()
        {
            byte result;
            var token = _tokenizer.NextToken(); // Freed at end of proc_instr
            if (token == null) return true;

            // Assembler Directives
            if (token.Equals("#CURFILE"))
            {
                _currentFile = _tokenizer.NextToken()!;
                _tokenizer.PrintedLineNumber = 1;
            }
            else if (token.Equals("#INCLUDE"))
            {
                // Include a file
                var fName = _tokenizer.NextToken()!; // ~Should~ be a filename
                var newPath = Path.Combine(Path.GetDirectoryName(_currentFile) ?? "", fName);
                var tempLines = File.ReadAllLines(newPath);
                _tokenizer.AddLines(new[] {"#CURFILE " + newPath});
                _tokenizer.AddLines(tempLines);
            }

            // "Real" Instructions
            else if (token.Equals("ADD"))
            {
                // ADD r1, r2, r0
                Output.Add(BaseInstructions.InstructionAdd(expect_reg(), expect_reg()));
            }
            else if (token.Equals("LIL"))
            {
                // LIL i,  r0
                result = 0x00;

                var t = next_num_or_label();
                if (t.Type == Parse.ParseType.Number)
                    result |= (byte) (t.Number & 0);
                else
                    _tokenizer.ThrowError("Expected number");

                result |= (byte) (expect_reg() << 4);
                Output.Add(result);
            }
            else if (token.Equals("LIH"))
            {
                // LIH i,  r0
                result = 0x40;

                var t = next_num_or_label();
                if (t.Type == Parse.ParseType.Number)
                    result |= (byte) ((t.Number >> 4) & 0x0F);
                else
                    _tokenizer.ThrowError("Expected number");

                result |= (byte) (expect_reg() << 4);
                Output.Add(result);
            }
            else if (token.Equals("STO"))
            {
                // STO rA, rB
                Output.Add(BaseInstructions.InstructionSto(expect_reg(), expect_reg()));
            }
            else if (token.Equals("LOA"))
            {
                // LOA rA, rB
                Output.Add(BaseInstructions.InstructionLoa(expect_reg(), expect_reg()));
            }
            else if (token.Equals("SKL"))
            {
                // SKL r0, r1
                Output.Add(BaseInstructions.InstructionSkl(expect_reg(), expect_reg()));
            }
            else if (token.Equals("INT") || token.Equals("EIN"))
            {
                // INT/EIN ii
                // TODO: FIX
                Output.Add(BaseInstructions.InstructionInt(expect_reg()));
            }
            else if (token.Equals("IOR"))
            {
                // IOR rA, rB
                Output.Add(BaseInstructions.InstructionIor(expect_reg(), expect_reg()));
            }
            else if (token.Equals("IOW"))
            {
                // IOW rA, rB
                Output.Add(BaseInstructions.InstructionIow(expect_reg(), expect_reg()));
            }
            else if (token.Equals("NAND"))
            {
                // NAND rA, rB
                Output.Add(BaseInstructions.InstructionNand(expect_reg(), expect_reg()));
            }

            // Pseudo-Instructions
            else if (token.Equals("LIR"))
            {
                // LIR ii, r0
                var t = next_num_or_label();
                var register = expect_reg();

                if (t.Type == Parse.ParseType.Label)
                {
                    ParseLabelLoad(t, register);
                }
                else
                {
                    Output.Add(BaseInstructions.PseudoLir(register, t.Number));
                }
            }

            else if (token[0] == '.')
            {
                // Direct Byte
                if (token[1] >= '0' && token[1] <= '9')
                {
                    result = ParseNumber(token[1..]);
                }
                else if (token[1] == '\'' && token[3] == '\'')
                {
                    // A character
                    result = (byte) (token[2] & 0xFF);
                }
                else
                {
                    throw Program.syntax_error("Expected number or single quoted character");
                }

                Output.Add(result);
            }
            else if (token[^1] == ':')
            {
                // Label Definitions
                LabelDef.Add(new Label(token[..^1], (byte) Output.Count, 0, 0));
            }
            else
            {
                Console.Error.WriteLine(" >> " + token + " <<");
                throw Program.syntax_error("Unknown token");
            }

            return false;
        }

        private byte expect_reg()
        {
            var r0 = next_num_or_label();
            if (r0.Type != Parse.ParseType.Reg)
            {
                Console.Error.WriteLine(r0.Type);
                Program.syntax_error("Register Expected");
            }

            var res = r0.Number;
            return res; // r0
        }

        // Get the next number or label
        private Parse next_num_or_label()
        {
            var token = _tokenizer.NextToken()!; // Freed at end of func

            var result = token[0] switch
            {
                '#' => new Parse(Parse.ParseType.Number, ParseNumber(token[1..])),
                '$' => new Parse(token[1..]),
                '%' => new Parse(Parse.ParseType.Reg, GetRegister(token)),
                _ => throw Program.syntax_error("Unknown reference!")
            };

            return result;
        }

        private static byte ParseNumber(string token)
        {
            // Handle hex
            if (token.Length >= 3 && (token[..2].Equals("0x") || token[..2].Equals("0X")))
                return Convert.ToByte(token[2..], 16);
            return byte.Parse(token, NumberStyles.Any);
        }

        // Get Single Register
        private static byte GetRegister(string token)
        {
            byte result = 0x00;

            if (token.Length != 3 || token[0] != '%')
            {
                Console.Error.WriteLine(" >> Expected register, got " + token + " <<");
                Program.syntax_error("Unknown Register");
            }

            switch (token[1])
            {
                case 'R':
                    break;
                case 'S':
                    result |= 0x08;
                    break;
                default:
                    Console.Error.WriteLine(" >> " + token + " <<");
                    Program.syntax_error("Unknown Register");
                    break;
            }

            if (token[2] >= '0' && token[2] <= '9')
            {
                result |= (byte) (token[2] - '0');
            }
            else
            {
                Console.Error.WriteLine(" >> " + token + " <<\n");
                Program.syntax_error("Unknown Register");
            }

            return result;
        }

        private void ParseLabelLoad(Parse value, byte reg)
        {
            if (value.Type != Parse.ParseType.Label)
                _tokenizer.ThrowError("Expected a label");
            LabelRef.Add(new Label(value.Name, (byte) Output.Count, 0xFF, 0));
            Output.Add(BaseInstructions.InstructionLil(reg, 0x00)); // LIL Low,  %R5
        }
    }
}