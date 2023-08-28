import { Expr } from "./stmts.ts";

export interface BinaryExpr extends Expr {
  type: "BinaryExpr";
  left: Expr;
  right: Expr | any;
  ooperator: string;
}

export interface AssignExpr extends Expr {
  type: "AssignExpr";
  assigne: Expr;
  value: Expr;
}

export interface MemberExpr extends Expr {
  type: "MemberExpr";
  obj: Expr;
  property: Expr;
  isIndexed: boolean;
}
export interface CallExpr extends Expr {
  type: "CallExpr";
  args: Expr[];
  caller: Expr;
}
