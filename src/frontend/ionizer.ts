import {Ion, Type} from "./Ion.ts";

export class Ionizer {
  private atoms;
  private ions: Ion[];
  line: number = 1;
  colmun: number = 1;

  constructor(atoms: string) {
    this.atoms = atoms.split("");
    this.ions = new Array<Ion>();
  }
  
  private KEYWORDS: Record<string, Type> = {
      set: Type.set_kw
  };

  ion(value, type: Type) : Ion {
    return {
      value,
      type,
      line: this.line,
      colmun: this.colmun
    };
  }

  isAllowedId(x: string) : boolean {
    return x.toUpperCase() != x.toLowerCase();
  }

  isNum(x: string) : boolean {
    return "0123456789٠١٢٣٤٥٦٧٨٩".includes(x);
  }

  isOOp(x: string) : boolean {
    return "+-*/%=<&".includes(x);
  }
  getLine(x: string) : boolean {
    if(x == "\n") {
      this.line++;
      this.colmun = 1;
      return true;
    }
    else {
      return false;
    }
  }
  isSkippableChar(x: string) {
    return "; ".includes(x) || x === "\t";
  }
  private add(value,type: Type) {
    if(value === undefined) {
      this.ions.push(this.ion(this.atoms.shift(), type));
    }
    else {
      this.ions.push(this.ion(value, type));
    }
    this.colmun++;
  }
  ionize(): Ion[] {
    
    while(this.atoms.length > 0) {
      switch(this.atoms[0]) {
       case "(" :
         this.add(undefined,Type.OpenParen);
         continue;
       case ")":
         this.add(undefined, Type.CloseParen);
         continue; 
       case "#":
         this.atoms.shift();
         while(!this.getLine(this.atoms[0]) && this.atoms.length > 0) {
          this.atoms.shift();
          this.colmun++;
         }
         continue;
       case "\n":
         this.getLine(this.atoms.shift());
         continue;
       default:
         if(this.isSkippableChar(this.atoms[0])) {
          this.atoms.shift();
          this.colmun++;
         }
         else if(this.isAllowedId(this.atoms[0])) {
          let res: string = "";
          while(this.atoms.length > 0 && this.isAllowedId(this.atoms[0])) {
            res += this.atoms.shift();
          }
          this.add(res, Type.id);
         }

         else {
           console.log("e");
           this.atoms.shift();
         }
         continue;
      }
    }
    this.add("END", Type.EOF);
    return this.ions;
  }

}

