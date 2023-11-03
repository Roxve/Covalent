open System;
open AtomicLang.Interpreter
open AtomicLang.Vals
printf ">> "
let code = Console.ReadLine();
let interpreter = new Interpreter(code);
let run : RuntimeVal = interpreter.run();

match run.Type with
| ValType.Num -> 
  let num = run :?> NumVal<float>
  printfn "%f" num.value
