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
type Num<'a>(aline : int, acolmun : int, value : 'a) as self= 
  interface Expr with
    member this.line = aline
    member this.colmun = acolmun

    member this.Type : NodeType = NodeType.Num
  end
  member x.getLine = (self :> Expr).line;
  member x.getColmun = (self :> Expr).colmun;

  member val value : 'a = value with get, set

[<StructuredFormatDisplay("got operator => value: {value}, line: {getLine}, colmun: {getColmun}")>]
type operator(aline : int, acolmun : int, value : string) as self= 
  interface Expr with
    member this.line = aline
    member this.colmun = acolmun

    member this.Type : NodeType = NodeType.Num
  end
  member x.getLine = (self :> Expr).line;
  member x.getColmun = (self :> Expr).colmun;

  member val value : string = value with get, set

[<StructuredFormatDisplay("got binary expression => value: {value}, line: {getLine}, colmun: {getColmun}")>]
type BinaryExpr(aline : int, acolmun : int, left : Expr, right : Expr, operator : operator) as self= 
  interface Expr with
    member this.line = aline
    member this.colmun = acolmun

    member this.Type : NodeType = NodeType.Num
  end
  member x.getLine = (self :> Expr).line;
  member x.getColmun = (self :> Expr).colmun;
  
  member val left : Expr = left with get, set
  member val right : Expr = right with get, set
  member val operator : operator = operator with get, set
