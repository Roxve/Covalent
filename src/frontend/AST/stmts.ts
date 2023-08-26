export type Node = 
  | "Program"
  | "VarCreation"
  | "FuncCreation"
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
export interface VarCreation extends Stmt {
  type: "VarCreation";
  name: string;
  isLocked: boolean;
  value: Expr;
}
// note: a function is an expr


// because exprs are stmts i guess
export interface Expr extends Stmt {}
