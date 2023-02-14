using System;
using System.Collections.Generic;

namespace BRISC8Assembler
{
    public static class Linker
    {
        public static void Link(List<byte> instructions, List<Label> definitions, List<Label> references)
        {
            // For every reference
            foreach (var reference in references)
            {
                // Find associated definition
                var definition = definitions.Find(l => l.Name.Equals(reference.Name));
                if (definition == null)
                {
                    Console.WriteLine(" >> " + reference.Name + " <<\n");
                    Program.syntax_error("Undefined Label");
                    return;
                }

                var offset = (ushort) (definition.Addr - reference.Addr - 3);
                instructions[reference.Addr] = (byte) (instructions[reference.Addr] | offset & 0x0F);
                instructions[reference.Addr+1] = (byte) (instructions[reference.Addr+1] | (offset >> 4) & 0x0F);
            }
        }
    }
}