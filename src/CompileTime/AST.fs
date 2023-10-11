namespace AtomicLang

type NodeType =
  | Program
  | Num
  | EOF

[<StructuredFormatDisplay("line: {line}, colmun: {colmun}")>]
type Expr =
  abstract member Type : NodeType with get
  abstract member line : int with get, set
  abstract member colmun : int with get, set
[<AbstractClass>]
[<StructuredFormatDisplay("got program => body: {body}, line: {line}, colmun: {colmun}")>]
type Program(aline : int, acolmun : int) =
  interface Expr with
    member val line = aline with get, set
    member val colmun = acolmun with get, set
    member this.Type = NodeType.Program
  end
  abstract member body : Expr list with get, set

[<AbstractClass>]
[<StructuredFormatDisplay("got num => value: {value}, line: {line}, colmun: {colmun}")>]
type Num(aline : int, acolmun : int ) = 
  interface Expr with
    member val line = aline with get, set
    member val colmun = acolmun with get, set

    member this.Type : NodeType = NodeType.Num
  end
  abstract member value : float with get, set
