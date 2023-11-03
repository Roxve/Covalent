import compile/tokenize

when isMainModule:
  stdout.write(">> ")
  stdout.flushFile

  var src = stdin.readLine()
  echo src
  var tokenizer = make_tokenizer(src)
  while(tokenizer.current_token.tok != TType.EOF):
    echo tokenizer.next
