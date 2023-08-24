import * as AST_S from "../AST/stmts.ts";
import { Stmt, Program } from "../AST/stmts.ts";
import * as AST_E from "../AST/exprs.ts";

import { extend } from "../etc.ts";
import { Ion, Type } from "../Ion.ts";

export class ParserMain {
   protected ions: Array<Ion>;
   protected line: number = 1;
   protected colmun: number = 1;
   

   public constructor(ions: Ion[]) {
      this.ions = ions;
   }

   protected take() : Ion | undefined {
      if(this.ions.length <= 0) {
         return {
            value: "END",
            type: Type.EOF,
            line: this.line,
            colmun: this.colmun
         } as Ion;
      }
      else {
         this.Update();
         return this.ions.shift();
      }
   }

   protected at() : Ion {
      if(this.ions.length <= 0) {
         return {
            value: "END",
            type: Type.EOF,
            line: this.line,
            colmun: this.colmun
         } as Ion;
      }
      else {
         this.Update();
         return this.ions[0];
      }
   }
   protected Update() {
     this.line = this.ions[0].colmun;
     this.colmun = this.ions[0].colmun;
   }
   
   protected notEOF(): boolean { 
      return this.at().type != Type.EOF;
   }
}
