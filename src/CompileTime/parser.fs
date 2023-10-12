namespace AtomicLang
open AtomicLang.lexer
open AtomicLang
module Parser =
    type Parser(code : string) = 
      let tokenizer : Tokenizer = new Tokenizer(code);
      do tokenizer.tokenize() |> ignore;

      let mutable line = 1
      let mutable colmun = 0

      let update() =
        line <- tokenizer.getLine
        colmun <- tokenizer.getColmun

      member private this.at() : Token =
        update |> ignore
        tokenizer.current_token()

      member private ths.take() : Token =
        let prev = tokenizer.current_token()
        tokenizer.tokenize() |> ignore
        update |> ignore
        prev
      member private this.notEOF() : bool =
        printfn "%A" (this.at())
        not (this.at().ttype = TokenType.EOF)

      member private this.parse_primary_expr() : AtomicLang.Expr =
        match this.at().ttype with
        | TokenType.Num -> new AtomicLang.Num(line,colmun, float(this.take().value))

      member private this.parse_expr : AtomicLang.Expr =
        this.parse_primary_expr()

       
      member this.productAST() : AtomicLang.Program =
        let prog = new AtomicLang.Program(line, colmun)
        while this.notEOF() do
          prog.body <- [this.parse_expr] |> List.append prog.body
        prog;
      
