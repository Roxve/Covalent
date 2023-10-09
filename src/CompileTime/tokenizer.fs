namespace AtomicLang
module lexer =

  type TokenType =
  | Num
  | Null
  | EOF

  type Token(value : string, ttype : TokenType, line : int, colmun : int) =
    member this.value = value;
    member this.ttype = ttype;
    member this.line = line;
    member this.colmun = colmun;

  type Tokenizer(code : string) = 
    let code = code;

    let take() : char =
      if code.Length <= 0 then
        ' ';
      else
        let prev = code[0];
        code = code.Substring(1) |> ignore;
        prev;

    let at() : char = 
      if code.Length <= 0 then 
        ' ';
      else
        code[0];
    let isNum() : bool = 
      "0123456789".Contains code[0];
    member this.line = 1;
    member this.colmun = 0;
  
    member this.tokenize() : Token list =
      let mutable tokens : Token list = []
      while code.Length > 0 do
        match at() with
        | ' ' | '\t' -> take() |> ignore;
        | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' ->
          let mutable res = "";
          while code.Length > 0 && isNum() do
            res <- res + string(take());
          tokens <- new Token(res, TokenType.Num, this.line, this.colmun) :: tokens;
        | _ -> printfn("invaild char.");
      tokens <- new Token("END", TokenType.EOF, this.line, this.colmun) :: tokens
      tokens;
