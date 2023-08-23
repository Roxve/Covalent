import {Ionizer} from "./frontend/ionizer.ts"

function main(args: string[]) {
  if(args === undefined || args === null || args.length <= 0) {
    Repl();
  }
}

function Repl() {
  console.log();
  while(true) {
    console.log("Atomic");
    const atoms: any = prompt("=>");
    var ionizer = new Ionizer(atoms);
    var ionized = ionizer.ionize();
    console.log(ionized);
  }
}
// Learn more at https://deno.land/manual/examples/module_metadata#concepts
if (import.meta.main) {
  main(Deno.args);
}
