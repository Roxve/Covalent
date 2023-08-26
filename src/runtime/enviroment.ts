import { RuntimeVal, ObjVal } from "./values.ts";
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
  
  public setObjProperty(obj_name: string, property: string, stmt: Stmt, index?: number) : RuntimeVal {
    let obj: ObjVal = this.findVar(obj_name, stmt) as ObjVal;
    if(index) {
      let key = Array.from(obj.value.keys())[index];

      return obj.value.get(key) || MK_NULL();
    }

    return obj.value.get(property) || MK_NULL();
  }
  public getObjProperty(obj_name: string, property: string, stmt: Stmt, index?: number) : RuntimeVal {
    let obj: ObjVal = this.findVar(obj_name, stmt) as ObjVal;
    if(index) {
      let key = Array.from(obj.value.keys())[index];

      return obj.value.get(key) || MK_NULL();
    }
    

    return obj.value.get(property) || MK_NULL();
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

  public findVar(name: string, stmt: Stmt) : RuntimeVal{
    const env = this.resolve(name, stmt);
    if(env === null) {
      return MK_NULL();
    }
    return env.vars.get(name) || MK_NULL();
  }
}
