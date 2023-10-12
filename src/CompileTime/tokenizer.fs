namespace AtomicLang
module lexer =

  type TokenType =
  | Num
  | Null
  | EOF
  [<StructuredFormatDisplay("line: {line}, colmun: {colmun}, type: {ttype}, value: {value}")>]
  type Token(value : string, ttype : TokenType, line : int, colmun : int) =
    member this.value = value;
    member this.ttype = ttype;
    member this.line = line;
    member this.colmun = colmun;

  type Tokenizer(code : string) as self = 
    let mutable line = 1;
    let mutable colmun = 0;

    let mutable code = code;
    
    let take() : char =
      if code.Length <= 0 then
        ' ';
      else
        colmun <- colmun + 1;
        let prev = code[0];
        printfn "%c"prev
        code <- code.Substring(1);
        prev;

    let at() : char = 
      if code.Length <= 0 then 
        ';';
      else
        code[0];
    let isNum() : bool = 
      "0123456789".Contains code[0];

    let mutable ccurrent_token : Token = new Token("unknown", TokenType.Null, -1, -1)
    member this.current_token() = ccurrent_token;

    member this.getLine with get () = line
    member this.getColmun with get () = colmun

    member x.set(value : string, ttype : TokenType) = 
      ccurrent_token <- new Token(value, ttype, line, colmun); 
      printfn "%A" (x.current_token())
      ignore

    member this.tokenize() : Token =
      while code.Length > 0 && at() = ' ' do take() |> ignore
      while code.Length > 0 && at() = '\n' do
        take() |> ignore
        line <- line + 1
        colmun <- 0
      if code.Length <= 0 then this.set("END", TokenType.EOF) |> ignore
      else
        match at() with
        | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' ->
          let mutable res = "";
          while code.Length > 0 && isNum() do
            res <- res + string(take())
            printfn "%s"res
          this.set(res, TokenType.Num) |> ignore
        | _ ->
          if at() = ' ' || at() = '\n' || at() = '\t' then
            this.tokenize() |> ignore
          else
            printfn "unexcepted char %c." (take());
      this.current_token();
