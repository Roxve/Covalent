import compile/parser
import print 
import compile/codegen
import runtime/vm
import noxen
import std/[times,os]
import etc/[enviroments, utils]
import strutils


proc main() =
  var args = commandLineParams()
  var src = ""
  if args.len <= 0:
    echo makeBox("welcome to the covalent repl!", "repl", full_style=noxen.green)  
    stdout.write(">> ".green)
    stdout.flushFile
    debug = true
    src = stdin.readLine()
    echo src

    
  else:
    case args[0]:
      of "run":
        if args.len < 2:
          discard
        else:
          var file_path = args[1]
          src = readFile(file_path)
          echo src
  debug = true
  let time = cpuTime()  
  var Parser = make_parser(src)
  var bytes = Parser.productBytes()
  print interpret(bytes)
  echo "- time: ", cpuTime() - time  
main()
