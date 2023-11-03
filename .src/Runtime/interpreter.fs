namespace AtomicLang
open AtomicLang.Parser
open AtomicLang.Vals
open AtomicLang.AST

module Interpreter=
  type Interpreter(code : string)=
    inherit InterpreterBinary.Interpreter(code)
    member x.run()=
      let mutable lastrun: RuntimeVal = new NullVal();
      while not (x.at().Type = AST.NodeType.EOP) do
        lastrun <- x.interpret (x.take())
        printfn "%A" lastrun
      lastrun
    member private x.interpret(expr : AST.Expr) : RuntimeVal =
      match expr.Type with
      | AST.NodeType.Num ->
        let num = expr :?> Num
        new NumVal<float>(num.value)
      | _ ->
        new NullVal();
