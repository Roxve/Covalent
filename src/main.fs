open System;
open AtomicLang.Parser

printf ">> "
let code = Console.ReadLine();
let parser = new Parser(code);
let AST = parser.productAST();


for expr in AST.body do
  printfn "%A" expr
