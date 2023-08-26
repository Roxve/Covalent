export type Node = 
  | "Program"
  | "VarCreation"
  | "FuncCreation"
  | "Num" 
  | "Str"
  | "Bool" 
  | "Null" 
  | "Id"
  | "Property"
  | "Obj" 
  | "BinaryExpr"
  | "AssignExpr";

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

export interface FuncCreation extends Stmt {
  type: "FuncCreation";
  name: string;
  body: Stmt[];
  parameters: string[];
}


// because exprs are stmts i guess
export interface Expr extends Stmt {}
