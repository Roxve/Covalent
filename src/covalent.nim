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
  var prog = Parser.productAST() 
  print prog
  for item in prog.body: 
    echo "try"
    print cast[Expr](item)
