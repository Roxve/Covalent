import compile/tokenize
import compile/parser
import print 
import compile/AST 
import compile/codegen

when isMainModule:
  stdout.write(">> ")
  stdout.flushFile

  var src = stdin.readLine()
  echo src
  var Parser = make_parser(src)
  var bytes = Parser.productBytes()
  print bytes
 # for item in prog.body:
  #  echo "try"
   # print item
