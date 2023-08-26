import { RuntimeVal } from "./values.ts";
import { Stmt } from "../frontend/AST/stmts.ts";
import { MK_NULL } from "./values.ts";
import { createError } from "../etc.ts";
export class Enviroment {
  locked_vars: Set<string>
  vars: Map<string, RuntimeVal>
  parent: Enviroment | null;
  
  private error(msg: string, code: string, stmt: Stmt) : void {
    createError(`Runtime: ${msg}\nat => line:${stmt.line}, colmun:${stmt.colmun},error code:${code}`);
  }
  public constructor(parent: Enviroment | null) {
    this.vars = new Map();
    this.locked_vars = new Set();
    this.parent = parent;
  }
  public resolve(name: string, stmt: Stmt) : Enviroment | null {
    if(this.vars.has(name)) {
      return this;
    }
    if(this.parent === null || this.parent === undefined) {
      this.error(`cannot resolve ${name}`, "AT2003", stmt);
      return null;
    }
    return this.parent?.resolve(name, stmt);
  }

  
  public declareVar(name: string,value: RuntimeVal,isLocked: boolean, stmt: Stmt) : RuntimeVal {
    if(this.vars.has(name)) {
      this.error(`var:${name} is already declared`, "AT2001", stmt);
      return MK_NULL();
    }
    
    this.vars.set(name, value);
    if(isLocked) {
      this.locked_vars.add(name);
    }
    return value;
  }
  

  public setVar(name: string, value: RuntimeVal,stmt: Stmt) : RuntimeVal {
    const env = this.resolve(name, stmt);
    if(env === null) {
      return MK_NULL();
    }
    if(env.locked_vars.has(name)) {
      this.error("cannot assinge value to a locked var", "AT2002", stmt);
      return MK_NULL();
    }
    env.vars.set(name, value);
    return value;
  }

  public findVar(name: string, stmt: Stmt) {
    const env = this.resolve(name, stmt);
    if(env === null) {
      return MK_NULL();
    }
    return env.vars.get(name);
  }
}
