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

  error(message: string) {
    console.log("%c" + message, 'background-color: gold; color: crimson');

    return {
      message,
      type: "error"
    };
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

  private take() {
    this.colmun++;
    return this.atoms.shift();
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
       case "{": 
         this.add(undefined, Type.OpenBrace);
         continue;
       case "}":
         this.add(undefined, Type.CloseBrace);
         continue;
       case ",":
         this.add(undefined, Type.Comma);
         continue;
       case ".": 
         this.add(undefined, Type.Dot);
         continue;
       case '"': 
       case "'":
        let char = this.take();
        let res = "";
        while(this.atoms[0] != char && this.atoms.length > 0) {
          res += this.take();
        }
        if(this.atoms[0] != char) {
          this.error("reached end of file and didnt finish string");
        }
        else {
          this.add(res, Type.str_type);
          this.take();
        }
        continue;
       case "#":
         this.take();
         while(!this.getLine(this.atoms[0]) && this.atoms.length > 0) {
          this.take();
         }
         continue;
       case "\n":
         this.getLine(this.take());
         continue;
       default:
         if(this.isSkippableChar(this.atoms[0])) {
          this.take();
         }
         else if(this.isAllowedId(this.atoms[0])) {
          let res: string = "";
          while(this.atoms.length > 0 && this.isAllowedId(this.atoms[0])) {
            res += this.take();
          }
          this.add(res, Type.id);
         }

         else {
           this.error("unknown char");
           this.take();
         }
         continue;
      }
    }

    this.add("END", Type.EOF);
    
    return this.ions;
  }

}

