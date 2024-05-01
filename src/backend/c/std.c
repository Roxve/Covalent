#include "std.h"
#include "stdio.h"
#include <stdarg.h>
#include <stdbool.h>
#include <string.h>


void *GC_malloc(unsigned int);
void *GC_realloc(void *, unsigned int);
void GC_free(void *);
void GC_init();

#define DEFOP_N(name, op)                                                      \
  Obj __##name##__(Obj a, Obj b) {                                          \
    __conv__(&a, &b);                                                          \
    TYPE kind = a.kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return __int__(a.val.i op b.val.i);                                    \
    case FLOAT_TYPE:                                                           \
      return __float__(a.val.f op b.val.f);                                  \
    default:                                                                   \
      return __NaN__();                                                        \
    }
#define DEFOP_NF(name, op)                                                      \
  Obj __##name##__(Obj a, Obj b) {                                          \
    __conv__(&a, &b);                                                          \
    TYPE kind = a.kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return __int__(a.val.i op b.val.i);                                    \
    default:                                                                  \
      return __NaN__();                                                        \
    }
    
#define DEFOP_LOGICAL(name, op)                                                      \
  Obj __##name##__(Obj a, Obj b) {                                          \
    __conv__(&a, &b);                                                          \
    TYPE kind = a.kind;                                                       \
    switch (kind) {                                                            \
    case BOOL_TYPE:                                                             \
      return __int__(a.val.b op b.val.b);                                    \
    default:                                                                  \
      return __NaN__();                                                        \
    }
    
#define DEFOP_BOOL(name, op)                                                   \
  _Bool __##name##__(Obj a, Obj b) {                                         \
    __conv__(&a, &b);                                                          \
    TYPE kind = a.kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return a.val.i op b.val.i;                                             \
    case FLOAT_TYPE:                                                           \
      return a.val.f op b.val.f;                                             \
    case BOOL_TYPE:                                                            \
      return a.val.b op b.val.b;                                             \
    case STR_TYPE:                                                             \
      return __str##name##__(a.val.s, b.val.s);                              \
    default:                                                                   \
      return 0;                                                        \
    }
#define DEFOP_STR(name, op)                                                    \
  Obj __##name##__(Obj a, Obj b) {                                          \
    __conv__(&a, &b);                                                          \
    TYPE kind = a.kind;                                                       \
    switch (kind) {                                                            \
    case INT_TYPE:                                                             \
      return __int__(a.val.i op b.val.i);                                    \
    case FLOAT_TYPE:                                                           \
      return __float__(a.val.f op b.val.f);                                  \
    case STR_TYPE:                                                             \
      return __str__(__str##name##__(a.val.s, b.val.s));                     \
    default:                                                                   \
      return __NaN__();                                                        \
    }

#define DEF(type, name, op)                                                    \
  DEFOP_##type(name, op);                                                      \
  }

Obj __NaN__() {
  Obj nan = (Obj){-1};
  return nan;
}

void __conv__(Obj *a, Obj *b) {
  TYPE a_ty = (a)->kind;
  TYPE b_ty = (b)->kind;

  Value a_val = a->val;
  Value b_val = b->val;

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

void writeln(Obj arg) {
  TYPE ty = arg.kind;
  switch (ty) {
  case INT_TYPE: {
    printf("%d\n", arg.val.i);
    break;
  }

  case FLOAT_TYPE: {
    printf("%f\n", arg.val.f);
    break;
  }
  case STR_TYPE: {
    printf("%.*s\n", arg.val.s->len, arg.val.s->val);
    break;
  }
  case BOOL_TYPE: {
    if (arg.val.b == 0) {
      printf("false\n");
    } else {
      printf("true\n");
    }
  }
  }
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

DEF(STR, add, +);

DEF(N, sub, -);

DEF(N, mul, *);

DEF(N, div, /);
DEF(NF, mod, %);

DEF(LOGICAL, and, &&);
DEF(LOGICAL, or, ||);

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

Str *__strclone__(Str *obj) {
  unsigned int size = STR_SIZE + (1 * obj->len);
  Str *cloned = GC_malloc(size);
  memcpy(cloned, obj, size);
  return cloned;
}

void __init__() { GC_init(); }

List* __listnew__(size_t elem_size, size_t size, ...) {
    void* arr = GC_malloc(elem_size * size);
    va_list args;
    va_start(args, size);

    char* next_ele = (char*) arr;
    for(int i = 0; i < size*elem_size/4; i++) {
      ((int*) arr)[i] = va_arg(args, int);
    }
    
    List* list = (List*)GC_malloc(sizeof(List));
    list->array = arr;
    list->elem_size = elem_size;
    list->size = size;
    return list;
}

// name and arguments to match type extending functions, generics for now is just Dynamic type
// set push(T): List(T) self, T item -> List(T)
List* Listdotpush(List *self, Obj item) {
  self->array = GC_realloc(self->array, self->size*self->elem_size+self->elem_size);
  void *insert = (char *) self->array + self->size * self->elem_size;
  memcpy(insert, &item.val, self->elem_size);

  return self;
}

// set pop(T): List(T) self -> List(T)
List* Listdotpop(List *self) {
  self->array = GC_realloc(self->array, self->size*self->elem_size-self->elem_size);
  return self;
}

void __free__(void *item) {
    GC_free(item);
}
