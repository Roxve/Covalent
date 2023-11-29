import print 
import compile/[parser,parse_expr, compiler,codegen]
import runtime/vm
import noxen
import std/[times,os]
import etc/[utils]


import compile/AST

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
  var parser = make_parser(src)
  # needed to tell the parser where the start is
  parser.parse_start = proc(self: var Parser): Expr = self.parse_expr
  
  var bytes = parser.productBytes()
  print interpret(bytes)
  echo "- time: ", cpuTime() - time  
main()
