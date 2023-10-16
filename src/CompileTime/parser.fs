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
        not (this.at().ttype = TokenType.EOF)
      member private this.parse_operator() : AtomicLang.operator =
        new AtomicLang.operator(line, colmun, this.take().value)
      member private this.parse_primary_expr() : AtomicLang.Expr =
        match this.at().ttype with
        | TokenType.Num -> 
          let num = float(this.take().value);
          //check if not convertable to int
          if (num - float(int(num))) > 0 then
            new AtomicLang.Num<float>(line,colmun, num)
          else
            new AtomicLang.Num<int>(line,colmun, int(num))

      member private this.parse_expr : AtomicLang.Expr =
        this.parse_primary_expr()

       
      member this.productAST() : AtomicLang.Program =
        let prog = new AtomicLang.Program(line, colmun)
        while this.notEOF() do
          prog.body <- [this.parse_expr] |> List.append prog.body
        prog;
      
