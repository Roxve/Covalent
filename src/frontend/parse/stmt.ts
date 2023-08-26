import { ParserMain } from "./main.ts";
import { Stmt, Expr, VarCreation } from "../AST/stmts.ts";
import { Null } from "../AST/values.ts";
import { Type } from "../Ion.ts";

export class ParserStmt extends ParserMain {
   parse_creation() : Stmt {
      this.except(Type.set_kw);
      
      let isLocked: boolean = false;
      if(this.at().type == Type.locked_kw) {
         this.take();

         isLocked = true;
      }
      let name = this.except(Type.id).value;
      switch(this.at().type) {
         
         default:
            return this.parse_var_creation(name, isLocked); 
      }
   }

   parse_var_creation(name: string, isLocked: boolean) : Stmt {
      let value: Expr;
      if(this.at().type != Type.setter) {
         if(isLocked) {
            this.error("must assinge value to locked var", "AT1006");
            return { type: "Null", value: null, line: this.line, colmun: this.colmun } as Null;
         }
         value = { type: "Null", value: null, line: this.line, colmun: this.colmun } as Null;
      }
      else {
         this.take();
         value = this.parse_expr();
      }

     return {
        type: "VarCreation",
        name: name,
        value: value,
        isLocked: isLocked,
        line: this.line,
        colmun: this.colmun
     } as VarCreation;
   }
}
