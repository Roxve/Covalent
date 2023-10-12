open System;
open AtomicLang.Parser

printf ">> "
let code = Console.ReadLine();
let parser = new Parser(code);
let AST = parser.productAST();

printfn "%A" AST
