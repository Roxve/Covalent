#include "stdbool.h"
typedef struct {
  char ty;
} Obj;
typedef struct Int {
  char ty;
  int val;
} Int;

typedef struct Str {
  char ty;
  char *val;
  int len;
} Str;

typedef struct Float {
  char ty;
  float val;
} Float;

typedef struct Bool {
  char ty;
  _Bool val;
} Bool;

typedef struct NaN {
  char ty;
} NaN;
void writeln(void *arg);
void *__int__(int i);
void *__float__(float f);
void *__str__(Str *s);
void *__bool__(_Bool b);

Str *__strnew__(char *val);

void err(char *err, int code);
void *__add__(void *a, void *b);
void *__sub__(void *a, void *b);
void *__mul__(void *a, void *b);
void *__div__(void *a, void *b);

Str *__stradd__(Str *a, Str *b);
