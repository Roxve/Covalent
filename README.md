# this is an unstable branch of a type rewrite, it will introduce big changes and it is not expected for it to compile


# Covalent
WIP DOP Compiled Language, with a cool type system that supports completly optional typing (no need to type even the function arguments) and a dynamic system(avoided as much as possible, may be discarded)

```
covalent path/to/file # builds results in ./ and C code in /tmp/covalent
```

to enter the repl
```
covalent
```
**desinged for linux for now**
**you need gcc installed in path to compile(repl included for now)**

**you only need the covalent exe and libs folder (they should be locatened in same folder) libs should contain runtime.o built from src/backend/c/std.c and gc.o, should be automatically built using cargo**

you can compile gc.o from [bdwgc](https://github.com/ivmai/bdwgc) (put it in libs folder) 

# features:
- C backend
- if-else
- while loops
- variables
- functions
- built-in types: Str, Int, Float, Bool, List(T), Any, Dynamic
- operators: -, +, *, /, %, &&, ||, ==, >, <, <=, >=

test project in /TestProj, examples in /code
