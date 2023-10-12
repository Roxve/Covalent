namespace AtomicLang

type NodeType =
  | Program
  | Num
  | EOF

[<StructuredFormatDisplay("line: {line}, colmun: {colmun}")>]
type Expr =
  abstract member Type : NodeType
  abstract member line : int
  abstract member colmun : int

[<StructuredFormatDisplay("got program => body: {body}, line: {getLine}, colmun: {getColmun}")>]
type Program(aline : int, acolmun : int)  as self =
  interface Expr with
    member this.line = aline
    member this.colmun = acolmun
    member this.Type = NodeType.Program
  end
  member x.getLine = (self :> Expr).line;
  member x.getColmun = (self :> Expr).colmun;

  member val body : Expr list = [] with get, set

[<StructuredFormatDisplay("got num => value: {value}, line: {getLine}, colmun: {getColmun}")>]
type Num(aline : int, acolmun : int, value : float) as self= 
  interface Expr with
    member this.line = aline
    member this.colmun = acolmun

    member this.Type : NodeType = NodeType.Num
  end
  member x.getLine = (self :> Expr).line;
  member x.getColmun = (self :> Expr).colmun;

  member val value : float = value with get, set
