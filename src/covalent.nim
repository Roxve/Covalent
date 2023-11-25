import compile/parser
import print 
import compile/codegen
import runtime/vm
import noxen
import std/times
import etc/[enviroments, utils]




proc main() =
  echo makeBox("welcome to the covalent repl!", "repl", full_style=noxen.green)  
  stdout.write(">> ".green)
  stdout.flushFile
  debug = true
  
  var src = stdin.readLine()
  echo src

  
  let time = cpuTime()  
  var Parser = make_parser(src)
  var bytes = Parser.productBytes()
  print interpret(bytes)
  echo "- time: ", cpuTime() - time  
# for item in prog.body:
  #  echo "try"
   # print item
main()
