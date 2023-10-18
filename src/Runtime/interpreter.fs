namespace AtomicLang
open AtomicLang.Parser
open AtomicLang.Vals
open AtomicLang.AST

module Interpreter=
  type Interpreter(code : string)=
    let parser = new Parser(code)
    let mutable line = 1
    let mutable colmun = 0
    do parser.productAST() |> ignore

    let update =
      line <- parser.current_node.line
      colmun <- parser.current_node.colmun
    member private x.at() = 
      update
      parser.current_node
     
    member private x.take() =
      let prev = x.at()
      parser.productAST() |> ignore
      update
      prev
    
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
