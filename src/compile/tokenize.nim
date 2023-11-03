type
  TType* = enum
    num,
    str,
    null,
    exception,
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
  let prev = self.src[0]
  
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
  if self.at() == ' ' or self.at() == ' ':
    while self.at() == ' ' or self.at() == '\t':
      discard self.take()

  if self.at() == '\n':
    self.line += 1
    self.colmun = 0

  if self.src.len <= 0: 
    return self.make("<EOF>", TType.EOF)

  case self.src[0]:
    of '0','1', '2', '3', '4', '5', '6', '7', '8', '9':
      var res = ""
      while self.src.len > 0 and isNum(self.at()):
        res &= self.take()
      return self.make(res, TType.num)

    of '"', "'"[0]:
      var op = self.take()
      var res = ""
      while self.src.len > 0 and self.at() != op:
        res &= self.take()

      if self.src.len <= 0 and self.at() != op:
        echo "error: reached end of file string didnt finish execepting " & op
        return self.make("unfinished_string", TType.exception)
      else:
        discard self.take
        return self.make(res,TType.str)
    else:
      echo "error unknown char " & self.take()
      return self.make("unknown_char", TType.exception)
