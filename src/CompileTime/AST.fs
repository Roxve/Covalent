namespace AtomicLang

type NodeType =
  | Program
  | Num
  | EOF

type Expr =
  abstract member Type : NodeType
  abstract member line : int with get, set
  abstract member colmun : int with get, set

type Program =
  inherit Expr
  abstract member Type : NodeType
  abstract member body : Expr list with get, set

type Num = 
  inherit Expr
  abstract member value : float with get, set
