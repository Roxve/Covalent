import boxen from "npm:boxen";

export var isError: boolean = false;
export var isTest: boolean = false;
export var currentPath: string = Deno.cwd();
export var mainPath: string = Deno.cwd();
export function setPath(dir: string) {
  currentPath = dir;
  if(isTest) {
    console.log("%clog: changing dir to:"+ dir, 'color: yellow');
  }
}
export function setMainPath(dir: string) {
  mainPath = dir;
}
export function setTest() {
  isTest = true;
}
export function setError() {
  isError = true;
}

export function createError(msg: string): string {
  let box = boxen(msg, { title: "error" });

  console.log(`%c${box}`, "color: crimson");
  setError();
  return box;
}

export function createWarning(msg: string): string {
  let box = boxen(msg, { title: "warning" });

  console.log(`%c${box}`, "color: yellow");

  return box;
}
