#include "stdbool.h"
#include "stdlib.h"

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

typedef struct IonTable {
  struct Ion *(*add)(struct Ion *self, struct Ion *b);
} IonTable;

// an Ion is an Atom everything is an Atom expect base types an Ion is a base
// type as an Atom
typedef struct Ion {
  TYPE kind;
  IonTable *table;
} Ion;

typedef struct {
  TYPE kind;
  IonTable *table;
  int val;
} IonizedInt;

IonizedInt *IonizedIntdotNew(int val);

void writeln(Obj arg);

#define __int__(int) ((Obj){INT_TYPE, (Value){.i = int}})
#define __float__(flo) ((Obj){FLOAT_TYPE, (Value){.f = flo}})
#define __str__(str) ((Obj){STR_TYPE, (Value){.s = str}})
#define __bool__(bool) ((Obj){BOOL_TYPE, (Value){.b = bool}})

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
Str *__strclone__(Str *obj);
void __init__();

#define __listget__(list, type, index) (((type *)(list)->array)[index])

typedef struct List {
  void *array;
  size_t size;
  size_t elem_size;
} List;

List *Listdotpush(List *self, Obj item);
List *__listnew__(size_t elem_size, size_t size, ...);
List *Listdotpop(List *self);

void __free__(void *item);

Str *itos(int i);