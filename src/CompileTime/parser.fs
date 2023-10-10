namespace AtomicLang
open AtomicLang.lexer

module Parser =
    type Parser(code : string) = 
      let mutable tokenizer : Tokenizer = new Tokenizer(code);
      do tokenizer.tokenize() |> ignore;

