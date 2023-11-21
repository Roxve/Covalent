import compile/tokenize
import compile/parser
import print 
import compile/AST 
import compile/codegen
import runtime/vm
import noxen

when isMainModule:
  
  echo makeBox("welcome to the covalent repl!", "repl", full_style=noxen.green)  
  stdout.write(">> ".green)
  stdout.flushFile
  
  var src = stdin.readLine()
  echo src
  var Parser = make_parser(src)
  var bytes = Parser.productBytes()
  print interpret(bytes)
# for item in prog.body:
  #  echo "try"
   # print item
