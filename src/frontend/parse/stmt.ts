import { ParserMain } from "./main.ts";
import { Stmt, Expr } from "../AST/stmts.ts";

export class ParserStmt extends ParserMain {
   //we just need a code that passes compie time here
   protected parse_stmt() : Stmt { 
      return {

      } as Stmt;
   }

   protected parse_expr() : Expr {
      return {

      } as Expr;
   }


}
