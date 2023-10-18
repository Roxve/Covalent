namespace AtomicLang
open AtomicLang.lexer
open AtomicLang.AST
module Parser =
    type Parser(code : string) = 
      let tokenizer : Tokenizer = new Tokenizer(code);
      do tokenizer.tokenize() |> ignore;

      let mutable line = 1
      let mutable colmun = 0
      let mutable pcurrent_node : Expr = new Null(-1,-1)
      let update() =
        line <- tokenizer.getLine
        colmun <- tokenizer.getColmun

      member this.current_node = pcurrent_node

      member private this.at() : Token =
        update |> ignore
        tokenizer.current_token()

      member private ths.take() : Token =
        let prev = tokenizer.current_token()
        tokenizer.tokenize() |> ignore
        update |> ignore
        prev
      member private this.notEOF() : bool =
        not (this.at().ttype = TokenType.EOF)

      member this.productProgram() : Program =
        let prog = new Program(line, colmun)
        while this.notEOF() do
          prog.body <- [this.parse_expr] |> List.append prog.body
        prog;


      member this.productAST() : Expr =
        if this.notEOF() then
          pcurrent_node <- this.parse_expr
          pcurrent_node;
        else
          new EOP(line, colmun);

      member private this.parse_operator : operator =
        new operator(line, colmun, this.take().value)
      member private this.parse_expr : Expr =
        this.parse_additive_binary_expr


      member private this.parse_additive_binary_expr =
        let mutable left = this.parse_multipictive_binary_expr
        while this.at().value = "+" || this.at().value = "-" do
          let op = this.parse_operator
          let right = this.parse_multipictive_binary_expr 
          left <- new BinaryExpr(line,colmun, left, right, op) :> Expr
        left
      

      member private this.parse_multipictive_binary_expr =
        let mutable left = this.parse_primary_expr
        while this.at().value = "*" || this.at().value = "/" || this.at().value = "%" do
          let op = this.parse_operator
          let right = this.parse_primary_expr 
          left <- new BinaryExpr(line,colmun, left, right, op) :> Expr
        left
 


      member private this.parse_primary_expr : Expr =
        match this.at().ttype with
        | TokenType.Num -> 
          let num = float(this.take().value);
          //check if not convertable to int
          new Num(line,colmun, num)
        | _ ->
          let tok = this.take().value 
          printfn "unexcepted Token %s" tok
          new Error(line, colmun, "unexcepted token " + tok)
