
type
  TType* = enum
    num,
    str,
    null, 
    EOF
  Token* = object
    line*, colmun*: int
    tok*: TType
    value*: string
  Tokenizer* = object
    current_token*: Token
    src: string
    line: int = 1
    colmun: int = 0

proc make_tokenizer*(src: string): Tokenizer =
  return Tokenizer(src: src)

proc make(self: var Tokenizer, value: string, tok: TType): Token =
  var token = Token(line: self.line, colmun: self.colmun, tok: tok, value: value)
  self.current_token = token
  return token


proc take(self: var Tokenizer): char = 
  let prev = self.src[self.src.len - 1]
  
  self.src = self.src[1 .. self.src.len - 1] 
  self.colmun += 1

  return prev 

proc isNum*(x: char): bool =
  return "01234.56789".contains(x);

proc at(self: var Tokenizer): char =
  if self.src.len > 0:
    return self.src[0]
  else:
    return '?'

proc next*(self: var Tokenizer): Token =
  case self.src[0]:
    of '0','1', '2', '3', '4', '5', '6', '7', '8', '9':
      var res = ""
      while self.src.len > 0 and isNum(self.at()):
        res &= self.take()
      return self.make(res, TType.num)

