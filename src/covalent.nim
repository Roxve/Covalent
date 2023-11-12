import compile/tokenize
import compile/parser
import print 
import compile/AST 

when isMainModule:
  stdout.write(">> ")
  stdout.flushFile

  var src = stdin.readLine()
  echo src
  var Parser = make_parser(src)
  var program = Parser.productAST()
  print program.prog
 # for item in prog.body:
  #  echo "try"
   # print item
