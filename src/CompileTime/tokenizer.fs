module AtomicLang
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

  member this.line = 1;
  member this.colmun = 0;
  
  member this.tokenize() : Token =


