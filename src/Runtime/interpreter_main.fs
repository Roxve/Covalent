namespace AtomicLang
open AtomicLang.Parser
module InterpreterMain=
  type Interpreter(code : string) =
    let parser = new Parser(code)
    let mutable line = 1
    let mutable colmun = 0
    do parser.productAST() |> ignore

    let update =
      line <- parser.current_node.line
      colmun <- parser.current_node.colmun
    member x.at() = 
      update
      parser.current_node
     
    member x.take() =
      let prev = x.at()
      parser.productAST() |> ignore
      update
      prev
    

