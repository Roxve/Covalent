import { ParserStmt } from "./stmt.ts";
import { Null, Num, Str, Bool, Id, Property, Object } from "../AST/values.ts";
import { Expr } from "../AST/stmts.ts";
import { BinaryExpr, AssignExpr, MemberExpr, CallExpr} from "../AST/exprs.ts";
import { Type, Ion } from "../Ion.ts";

export class ParserExpr extends ParserStmt {

   protected parse_assign_expr() : Expr {
      const left = this.parse_obj_expr();

      if(this.at().type === Type.equals) {
         this.take();

         const value = this.parse_expr();
         
         return {
            type: "AssignExpr",
            assigne: left,
            value: value,
            line: this.line,
            colmun: this.colmun
         } as AssignExpr;
      }

      return left;
   }



   protected parse_obj_expr() : Expr {
      if(this.at().type != Type.OpenBrace) {
         return this.parse_mathmatic_expr();
      }

      this.take();

      let properties: Property[] = [];

      while(this.notEOF() && this.at().type != Type.CloseBrace) {
         let key = this.except(Type.id).value;


         let property: Property;
         
         if(this.at().type === Type.Comma) {
            this.take();
            property = { 
               type: "Property", 
               key: key, 
               value: null,
               line: this.line,
               colmun: this.colmun
            }
            properties.push(property);
            continue;
         }
         else if(this.at().type === Type.CloseBrace) {
            property = {
               type: "Property",
               key: key,
               value: null,
               line: this.line,
               colmun: this.colmun
            }
            properties.push(property);
            continue;
         }
         this.except(Type.Colon);

         let value = this.parse_expr();

         property = {
            type: "Property",
            key: key,
            value: value,
            line: this.line,
            colmun: this.colmun
         }

         properties.push(property);

         if(this.at().type != Type.CloseBrace) {
            this.except(Type.Comma);
         }
      }
      this.except(Type.CloseBrace);

      let obj: Object = {
         type: "Obj",
         properties: properties,
         line: this.line,
         colmun: this.colmun
      }
      return obj;
   }



   protected parse_mathmatic_expr() : Expr {
      const main = this;
      function parse_additive_expr() : Expr {
         let left = parse_multiplactive_expr();
         
         while(main.at().value === "+" || main.at().value === "-") {
            let val: string = main.take().value;
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
         let left = main.parse_call_member_expr();
         
         while(main.at().value === "*" || main.at().value === "/" || main.at().value === "%") {
            let val: string = main.take().value;
            const right = main.parse_call_member_expr();
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

   protected parse_call_member_expr() : Expr {
      let member = this.parse_member_expr();

      if(this.at().type === Type.OpenParen) {
         return this.parse_call_expr(member);
      }

      return member;
   }
   
   protected parse_call_expr(caller: Expr) : Expr{
      let callExpr: Expr = {
         type: "CallExpr",
         caller,
         args: this.parse_args(),
         line: this.line,
         colmun: this.colmun
      } as CallExpr;
      //dont know why i did this? but it doesnt work without it? 
      if(this.at().type === Type.OpenParen) { 
         callExpr = this.parse_call_expr(callExpr);
      }
      return callExpr;
   }
   

   protected parse_args() : Expr[] {
      this.except(Type.OpenParen);

      let args: Expr[] = [];

      if(this.at().type != Type.CloseParen) {
         args = this.parse_args_list();
      }
      this.except(Type.CloseParen);

      return args;
   } 
   protected parse_args_list() : Expr[] {
      let args: Expr[] = [];
      args.push(this.parse_assign_expr());

      while(this.at().type === Type.Comma) {
         this.take();
         args.push(this.parse_assign_expr());
      }

      return args;
   }


   protected parse_member_expr() : Expr {
      let left = this.parse_primary_expr();

      while(this.notEOF() && (this.at().type === Type.Dot || this.at().type === Type.OpenBracket)) {
         let ooperator = this.take();
         let property: Expr; 
         let isIndexed: boolean; //indexed => obj[index], !indexed obj.property

         if(ooperator.type === Type.Dot) {
            isIndexed = false;

            property = this.parse_primary_expr();
            if(property.type != "Id") {
               this.error("excepted id of a property in a non indexed member expr", "AT1007");
               return { type: "Null", value: null } as Null;
            }
          }
            //excepts '['
            else {
               isIndexed = true;
               property = this.parse_expr();

               this.except(Type.CloseBracket);
            }
            left = {
             type: "MemberExpr",
             obj: left,
             property,
             isIndexed,
             line: this.line,
             colmun: this.colmun
            } as MemberExpr;
      }
      return left;
   }


   protected parse_primary_expr() : Expr {
      switch(this.at().type) { 
         case Type.OpenParen:
            this.take();
            let expr = this.parse_expr();
            this.except(Type.CloseParen);
            return expr;
         case Type.id:
            return {
               type: "Id",
               symbol: this.take().value,
               line: this.line,
               colmun: this.colmun
            } as Id;
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
            this.error("unexcepted ION", "AT1001");
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
