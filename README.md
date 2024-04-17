# Covalent
(WIP Language)
```
covalent path/to/file # builds results in ./
```

to enter the repl
```
covalent
```
(REPEL IS VERY WIP IT BUILDS EXE NAMED "repl" EVERY RUN IN CURRENT DIR)

**you need gcc installed in path to compile**

**you only need the covalent exe and libs folder (they should be locatened in same folder) libs should contain runtime.o built from src/backend/c/std.c should be automatically built using cargo**

you also need gc.o compile it from [bdwgc](https://github.com/ivmai/bdwgc) (put it in libs folder) 
to compile gc.o use this command 
```
gcc -I include -DUSE_MMAP_ANON -DUSE_MUNMAP -c extra/gc.c
```

TODO:
- automatic gc.o build script
