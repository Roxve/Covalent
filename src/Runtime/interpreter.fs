namespace AtomicLang
open AtomicLang.Parser


module Interpreter=
  type Interpreter(code : string)=
    let parser = new Parser(code)
    let mutable line = 1
    let mutable colmun = 0

    let update =
      line <- parser.current_node.line
      colmun <- parser.current_node.colmun
    let at = 
      update
      parser.current_node
     
    let take =
      let prev = at
      parser.productAST() |> ignore
      update
      prev
    
    member x.run()=
      while not (at.Type = AST.NodeType.EOP) do
        x.interpret take
      ignore
    member private x.interpret(expr : AST.Expr)=
      match expr.Type 
