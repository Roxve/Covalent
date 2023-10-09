open System;
open AtomicLang.lexer

printf ">> "
let code = Console.ReadLine();
let tokenizer = new Tokenizer(code);
let tokens = tokenizer.tokenize();

printfn "%A" tokens
