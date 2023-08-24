export type Node = 
  | "Program" 
  | "Num" 
  | "Str"
  | "Bool" 
  | "Null" 
  | "Id" 
  | "BinaryExpr";

export interface Stmt {
  type: Node;
  line: number;
  colmun: number;
}

export interface Program extends Stmt {
  type: "Program";
  body: Stmt[];
}

export interface Expr extends Stmt {}
