# Covalent
(WIP Language)
```
covalent path/to/file # builds results in ./
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
- built-in types: str, int, float, bool
- basic static until it has to be dynamic type-system(1/5 of planned implented) with bdwgc
