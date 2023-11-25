type
  TType* = enum
    id,
    operator,
    num,
    str,
    null,
    exception,
    # keywords
    set_kw = "set", 
    # new keywords should be put here dont replace set and null location
    to_kw = "to",
    true_kw = "true", false_kw = "false"
    null_kw = "null",
    colon,
    assign,
    openParen,
    closeParen,
    # end
    EOF
  Token* = object
    line*, colmun*: int
    tok*: TType
    value*: string
  Tokenizer* = object
    current_token*: Token
    src: string
    line*: int = 1
    colmun*: int = 0

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

proc isOperator(x: char): bool =
  return "+-*/=%|&<>^".contains(x)

proc isAllowedID(x: char): bool =
  return not "¿? \t\n".contains(x) and not x.isOperator # every char is vaild except <=

  
proc getKeywordID(x: string): TType =
  for keyword in ord(TType.set_kw) .. ord(TType.null_kw):
    if $TType(keyword) == x:
      return TType(keyword)
  return TType.id


proc at(self: var Tokenizer): char =
  if self.src.len > 0:
    return self.src[0]
  else:
    return '?'

proc next*(self: var Tokenizer): Token =
  # skip spaces and check if end of file
  if self.at() == ' ' or self.at() == ' ':
    while self.at() == ' ' or self.at() == '\t':
      discard self.take()

  while self.at() == '\n':
    self.line += 1
    self.colmun = 0
    discard self.take

  if self.src.len <= 0: 
    return self.make("<EOF>", TType.EOF)


  # check char by char
  case self.src[0]:
    # number
    of '0','1', '2', '3', '4', '5', '6', '7', '8', '9':
      var res = ""
      while self.src.len > 0 and isNum(self.at()):
        res &= self.take()
      return self.make(res, TType.num)
    # operators
    of '+', '-', '*', '/','=', '%', '<', '>', '&', '|', '^':
      
  
      if self.src[0] == '-':
        if self.src.len > 1 and isNum(self.src[1]): 
          var res = "-"
          discard self.take()
          while self.src.len > 0 and isNum(self.at()):
            res &= self.take()
          return self.make(res, TType.num)      
      
  
      var op: string = ""
      while self.src.len > 0 and self.at().isOperator:
        op &= self.take
      return self.make(op, TType.operator)
    # string
    of '"', "'"[0]:
      var op = self.take
      var res = ""
      while self.src.len > 0 and self.at() != op:
        res &= self.take()

      if self.src.len <= 0 and self.at() != op:
        echo "error: reached end of file string didnt finish execepting " & op
        return self.make("unfinished_string", TType.exception)
      else:
        discard self.take
        return self.make(res,TType.str)
    of '(':
      return self.make($self.take, openParen)
    of ')':
      return self.make($self.take, closeParen)
    of ':':
      discard self.take
      if self.at == '=':
        discard self.take
        return self.make(":=", assign)
      return self.make(":", colon)
    else:
      if self.at().isAllowedID:
        var res = ""
        while self.src.len > 0 and self.at().isAllowedID:
          res &= self.take()
        return self.make(res, getKeywordID(res))
      # unknown
      echo "error unknown char " & self.take()
      return self.make("unknown_char", TType.exception)
