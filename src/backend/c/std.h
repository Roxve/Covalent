#include "stdbool.h"

void GC_free(void *);
#define free(a) GC_free(a) // remove later

#define INT_TYPE 0
#define FLOAT_TYPE 1
#define STR_TYPE 2
#define BOOL_TYPE 3
typedef char TYPE;

typedef struct Str {
  char *val;
  int len;
} Str;

typedef union Value {
  int i;
  float f;
  _Bool b;
  Str *s;
} Value;

#define INT_SIZE sizeof(TYPE) + sizeof(int)
#define FLOAT_SIZE sizeof(TYPE) + sizeof(float)
#define STR_SIZE sizeof(TYPE) + sizeof(Str)
#define BOOL_SIZE sizeof(TYPE) + sizeof(_Bool)

typedef struct {
  TYPE kind;
  Value val;
} Obj;

void writeln(Obj arg);

#define __int__(int) ((Obj) {INT_TYPE, (Value) {.i = int}})
#define __float__(flo) ((Obj) {FLOAT_TYPE, (Value) {.f = flo}})
#define __str__(str) ((Obj) {STR_TYPE, (Value) {.s = str}})
#define __bool__(bool) ((Obj) {BOOL_TYPE, (Value) {.b = bool}})


Str *__strnew__(char *val);

void err(char *err, int code);
Obj __add__(Obj a, Obj b);
Obj __sub__(Obj a, Obj b);
Obj __mul__(Obj a, Obj b);
Obj __div__(Obj a, Obj b);

_Bool __comp__(Obj a, Obj b);
_Bool __ecomp__(Obj a, Obj b);
_Bool __eq__(Obj a, Obj b);

_Bool __strcomp__(Str *a, Str *b);
_Bool __strecomp__(Str *a, Str *b);
_Bool __streq__(Str *a, Str *b);

Str *__stradd__(Str *a, Str *b);
// Obj *__clone__(Obj *obj);
Str *__strclone__(Str *obj);
void __init__();
