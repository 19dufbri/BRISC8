﻿using System;
using System.IO;

namespace BRISC8Assembler
{
    internal static class Program
    {
        private static Tokenizer _tokenizer;
        
        private static void Main(string[] args)
        {
            // Sanity check
            if (args.Length < 2)
            {
                Console.Error.WriteLine("Not Enough Arguments!");
                Console.Error.WriteLine("Usage: " + Environment.CommandLine + " <input file> <output file>");
                return;
            }

            // Read input file and tokenize it
            var currentFile = args[0];

            _tokenizer = new Tokenizer(Array.Empty<string>());
            for (var i = 0; i < args.Length - 1; i++)
                _tokenizer.AddLines(File.ReadLines(args[i]));

            var generator = new Generator(_tokenizer, currentFile);

            // Create unlinked instruction stream
            generator.GenerateAll();

            // Linking
            Linker.Link(generator.Output, generator.LabelDef, generator.LabelRef);

            var output = generator.Output;

            // Write the final program to a file
            File.WriteAllBytes(args[^1], output.ToArray());
            Console.WriteLine("Assembly Successful");
        }

        public static Exception syntax_error(string error)
        {
            throw new Exception("Assembly Error: " + error + " at " + _tokenizer.CurrentLine());
        }
    }
}