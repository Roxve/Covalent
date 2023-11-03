namespace AtomicLang
module InterpreterBinary=
  type Interpreter(code : string)=
    inherit InterpreterMain.Interpreter(code)
