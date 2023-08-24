import { RuntimeVal } from "./values.ts";

export class Enviroment {
  locked_vars: Set<string>
  vars: Map<string, RuntimeVal>
  parent: Enviroment | null;
  
  public constructor(parent: Enviroment | null) {
    this.vars = new Map();
    this.locked_vars = new Set();
    this.parent = parent;
  }

}
