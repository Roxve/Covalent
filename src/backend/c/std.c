#include "std.h"
#include "stdio.h"
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

void *GC_malloc(unsigned int);
void GC_free(void *);
void GC_init();

#define DEFOP_N(name, op)                                                      \
  Obj *__##name##__(Obj *a, Obj *b) {                                          \
    __conv__(&a, &b);                                                          \
    TYPE kind = a->kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return __int__(a->val.i op b->val.i);                                    \
    case FLOAT_TYPE:                                                           \
      return __float__(a->val.f op b->val.f);                                  \
    default:                                                                   \
      return __NaN__();                                                        \
    }
#define DEFOP_BOOL(name, op)                                                   \
  _Bool __##name##__(Obj *a, Obj *b) {                                         \
    __conv__(&a, &b);                                                          \
    TYPE kind = a->kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return a->val.i op b->val.i;                                             \
    case FLOAT_TYPE:                                                           \
      return a->val.f op b->val.f;                                             \
    case BOOL_TYPE:                                                            \
      return a->val.b op b->val.b;                                             \
    case STR_TYPE:                                                             \
      return __str##name##__(a->val.s, b->val.s);                              \
    default:                                                                   \
      return __NaN__();                                                        \
    }
#define DEFOP_STR(name, op)                                                    \
  Obj *__##name##__(Obj *a, Obj *b) {                                          \
    __conv__(&a, &b);                                                          \
    TYPE kind = a->kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return __int__(a->val.i op b->val.i);                                    \
    case FLOAT_TYPE:                                                           \
      return __float__(a->val.f op b->val.f);                                  \
    case STR_TYPE:                                                             \
      return __str__(__str##name##__(a->val.s, b->val.s));                     \
    default:                                                                   \
      return __NaN__();                                                        \
    }

#define DEF(type, name, op)                                                    \
  DEFOP_##type(name, op);                                                      \
  }

Obj *__NaN__() {
  Obj *nan = (Obj *)GC_malloc(sizeof(TYPE));
  nan->kind = -1;
  return nan;
}

void __conv__(Obj **a, Obj **b) {
  TYPE a_ty = (*a)->kind;
  TYPE b_ty = (*b)->kind;

  Value a_val = (*a)->val;
  Value b_val = (*b)->val;

  if (a_ty == b_ty) {
    return;
  }
  if (a_ty == FLOAT_TYPE && b_ty == INT_TYPE) {
    *b = __float__((float)b_val.i);
  } else if (a_ty == INT_TYPE && b_ty == FLOAT_TYPE) {
    *a = __float__((float)a_val.i);
  } else {
    err("cannot conv balance a and b", 5);
  }
}

void writeln(Obj *arg) {
  TYPE ty = arg->kind;
  switch (ty) {
  case INT_TYPE: {
    ;
    printf("%d\n", arg->val.i);
    break;
  }

  case FLOAT_TYPE: {
    printf("%f\n", arg->val.f);
    break;
  }
  case STR_TYPE: {
    Str *s = (Str *)arg;
    printf("%.*s\n", arg->val.s->len, arg->val.s->val);
    break;
  }
  case BOOL_TYPE: {
    if (arg->val.b == 0) {
      printf("false\n");
    } else {
      printf("true\n");
    }
  }
  }
}

// TODO: replace GC_malloc for dynamic consts into alloca
Obj *__int__(int i) {
  Obj *obj = (Obj *)GC_malloc(INT_SIZE);
  obj->kind = INT_TYPE;
  obj->val.i = i;
  return obj;
}

Obj *__str__(Str *s) {
  Obj *str = (Obj *)GC_malloc(STR_SIZE);
  str->kind = STR_TYPE;
  str->val.s = s;
  return str;
}

Obj *__bool__(_Bool b) {
  Obj *obj = (Obj *)GC_malloc(BOOL_SIZE);
  obj->kind = BOOL_TYPE;
  obj->val.b = b;
  return obj;
}

Str *__strnew__(char *s) {
  int len = strlen(s);
  char *str = (char *)GC_malloc(len);
  memcpy(str, s, len);

  Str *obj = (Str *)GC_malloc(sizeof(Str));

  obj->val = str;
  obj->len = len;
  return obj;
}

Obj *__float__(float f) {
  Obj *obj = (Obj *)GC_malloc(FLOAT_SIZE);
  obj->kind = FLOAT_TYPE;
  obj->val.f = f;
  return obj;
}
DEF(STR, add, +);

DEF(N, sub, -);

DEF(N, mul, *);

DEF(N, div, /);

DEF(BOOL, eq, ==);

DEF(BOOL, comp, >);

DEF(BOOL, ecomp, >=);

_Bool __streq__(Str *a, Str *b) {
  if (a->len != b->len) {
    return false;
  }

  char *str_a = a->val;
  char *str_b = b->val;
  for (int i = 0; i < a->len; i++) {
    if (str_a[i] != str_b[i]) {
      return false;
    }
  }
  return true;
}

_Bool __strcomp__(Str *a, Str *b) {
  int len_a = a->len;
  int len_b = b->len;
  return len_a > len_b;
}

_Bool __strecomp__(Str *a, Str *b) {
  int len_a = a->len;
  int len_b = b->len;
  char *str_a = a->val;
  char *str_b = b->val;
  return len_a > len_b || str_a == str_b;
}

Str *__stradd__(Str *a, Str *b) {
  int len = a->len + b->len;
  char *str = (char *)GC_malloc(len);
  memcpy(str, a->val, a->len);
  memcpy(str + a->len, b->val, b->len);

  Str *s = (Str *)GC_malloc(sizeof(Str));

  s->val = str;
  s->len = len;
  return s;
}

void err(char *err, int code) {
  printf("covalent runtime error: %s", err);
  exit(code);
}

unsigned int type_size(TYPE ty) {
  switch (ty) {
  case INT_TYPE:
    return INT_SIZE;
  case FLOAT_TYPE:
    return FLOAT_SIZE;
  case STR_TYPE:
    return STR_SIZE;
  case BOOL_TYPE:
    return BOOL_SIZE;
  default:
    err("UNKNOWN TYPE", 3);
    return sizeof(Obj);
  }
}

Obj *__clone__(Obj *obj) {
  TYPE ty = obj->kind;
  unsigned int size = type_size(ty);
  void *cloned = GC_malloc(size);
  memcpy(cloned, obj, size);
  return cloned;
}

void __init__() { GC_init(); }
