import { ParserStmt } from "./stmt.ts";
import { Null } from "../AST/values.ts";
import { Expr } from "../AST/stmts.ts";

export class ParserExpr extends ParserStmt {
   protected parse_primary_expr() : Expr {
      switch(this.at().type) {
         default:
            this.take();
            return {
               type: "Null",
               value: null,
               line: this.line,
               colmun: this.colmun
            } as Null;
      }
   }
}
