import { Expr } from "./stmts.ts";

export interface BinaryExpr extends Expr {
    type: "BinaryExpr";
    left: Expr;
    right: Expr;
    ooperator: string;
}
