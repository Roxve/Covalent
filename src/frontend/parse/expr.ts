import { ParserStmt } from "./stmt.ts";
import { Null, Num, Str, Bool } from "../AST/values.ts";
import { Expr } from "../AST/stmts.ts";
import { BinaryExpr } from "../AST/exprs.ts";
import { Type, Ion } from "../Ion.ts";

export class ParserExpr extends ParserStmt {
   protected parse_mathmatic_expr() : Expr {
      const main = this;
      function parse_additive_expr() : Expr {
         let left = parse_multiplactive_expr();
         const val = main.at().value;
         if(val === "+" || val === "-") {
            main.take();
            const right = parse_multiplactive_expr();
            left = {
               type: "BinaryExpr",
               left: left,
               ooperator: val,
               right: right,
               line: main.line,
               colmun: main.colmun
            } as BinaryExpr;
         }
         return left;
      }


      function parse_multiplactive_expr() : Expr {
         let left = main.parse_primary_expr();
         const val = main.at().value;
         if(val === "*" || val === "/" || val === "%") {
            main.take();
            const right = main.parse_primary_expr();
            left = {
               type: "BinaryExpr",
               left: left,
               ooperator: val,
               right: right,
               line: main.line,
               colmun: main.colmun
            } as BinaryExpr;
         }
         return left;
      }

      return parse_additive_expr();
   }


   protected parse_primary_expr() : Expr {
      switch(this.at().type) { 
         case Type.OpenParen:
            this.take();
            let expr = this.parse_expr();
            this.except(Type.CloseParen);
            return expr;
         case Type.str_type:
            return {
               type: "Str",
               value: this.take().value,
               line: this.line,
               colmun: this.colmun
            } as Str;
         case Type.num_type:
            return {
               type: "Num",
               value: +this.take().value,
               line: this.line,
               colmun: this.colmun
            } as Num;
         case Type.bool_type:
            return {
               type: "Bool",
               value: this.take().value == "true" ? true : false,
               line: this.line,
               colmun: this.colmun
            } as Bool;
         case Type.null_type:
            this.take();
            return {
               type: "Null",
               value: null,
               line: this.line,
               colmun: this.colmun
            } as Null;
         default:
            this.error("unexcepted ION", "AT0001");
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
