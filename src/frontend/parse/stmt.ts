import { ParserMain } from "./main.ts";
import { Stmt, Expr, VarCreation, FuncCreation , ReturnStmt} from "../AST/stmts.ts";
import { Null, Id } from "../AST/values.ts";
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
         case Type.OpenParen:
            if(isLocked) { 
              this.error("cannot declare a locked func because its already locked! did you mean: private", "AT1011");
              return { type: "Null", value: null } as Null;
            }
            return this.parse_func_creation(name);
            break;
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
        name,
        value,
        isLocked,
        line: this.line,
        colmun: this.colmun
     } as VarCreation;
   }

   parse_func_creation(name: string) : Stmt {
     let args: Expr[] = this.parse_args(); 
     let parameters: string[] = [];

     for(let arg of args) {
        if(arg.type != "Id") {
           this.error("inside function creation parameters has to be ids", "AT1012");
           return { type: "Null", value: null } as Null;
        }
        parameters.push((arg as Id).symbol);
     }
     this.except(Type.Colon);
     this.except(Type.OpenBrace);

     let body: Stmt[] = [];

     while(this.notEOF() && this.at().type != Type.CloseBrace) {
         body.push(this.parse_stmt());
     }

     this.except(Type.CloseBrace);
     return {
      type: "FuncCreation",
      name,
      parameters,
      body,
      line: this.line,
      colmun: this.colmun
     } as FuncCreation;
   }


   parse_return_stmt() : Stmt {
      this.take();

      let value = this.parse_expr();

      return {
         type: "ReturnStmt",
         value,
         line: this.line,
         colmun: this.colmun
      } as ReturnStmt;
   }
}
