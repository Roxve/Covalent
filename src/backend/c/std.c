#include "std.h"
#include "stdio.h"
#include <stdlib.h>
#include <string.h>

#define INT_TYPE 0
#define FLOAT_TYPE 1
#define STR_TYPE 2
#define BOOL_TYPE 3

void *__NaN__() {
  NaN *nan = (NaN *)malloc(sizeof(NaN));
  nan->ty = -1;
  return nan;
}

void __conv__(void **a, void **b) {
  char a_ty = ((Obj *)*a)->ty;
  char b_ty = ((Obj *)*b)->ty;
  if (a_ty == b_ty) {
    return;
  }
  if (a_ty == FLOAT_TYPE && b_ty == INT_TYPE) {
    int val = (((Int *)*b)->val);

    free(*b);
    *b = __float__((float)val);
  } else if (a_ty == INT_TYPE && b_ty == FLOAT_TYPE) {
    int val = (((Int *)*a)->val);

    free(*a);
    *a = __float__((float)val);
  } else {
    err("cannot conv balance a and b", 5);
  }
}

void writeln(void *arg) {
  char ty = ((Obj *)arg)->ty;
  switch (ty) {
  case INT_TYPE: {
    Int *i = (Int *)arg;
    printf("%d\n", i->val);
    break;
  }

  case FLOAT_TYPE: {
    Float *f = (Float *)arg;
    printf("%f\n", f->val);
    break;
  }
  case STR_TYPE: {
    Str *s = (Str *)arg;
    printf("%.*s\n", s->len, s->val);
    break;
  }
  case BOOL_TYPE: {
      Bool *b = (Bool *)arg;
      if (b->val == 0) {
        printf("false\n");
      } else {
        printf("true\n");
      }
    }
  }
}

void *__int__(int i) {
  Int *obj = (Int *)malloc(sizeof(Int));
  obj->ty = INT_TYPE;
  obj->val = i;
  return obj;
}

void *__str__(Str *s) {
  void *str = s;
  return str;
}

void *__bool__(_Bool b) {
  Bool *obj = (Bool *)malloc(sizeof(Bool));
  obj->ty = BOOL_TYPE;
  obj->val = b;
  return obj;
}

Str *__strnew__(char *s) {
  int len = strlen(s);
  char *str = (char *)malloc(len);
  memcpy(str, s, len);

  Str *obj = (Str *)malloc(sizeof(Str));
  obj->ty = STR_TYPE;

  obj->val = str;
  obj->len = len;
  return obj;
}

void *__float__(float f) {
  Float *obj = (Float *)malloc(sizeof(Float));
  obj->ty = FLOAT_TYPE;
  obj->val = f;
  return obj;
}

void *__add__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __int__(((Int *)a)->val + ((Int *)b)->val);
  case FLOAT_TYPE:
    return __float__(((Float *)a)->val + ((Float *)b)->val);
  case STR_TYPE:
    return __stradd__((Str *)a, (Str *)b);
  default:
    return __NaN__();
  }
}

void *__sub__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __int__(((Int *)a)->val - ((Int *)b)->val);
  case FLOAT_TYPE:
    return __float__(((Float *)a)->val - ((Float *)b)->val);
  default:
    return __NaN__();
  }
}

void *__mul__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __int__(((Int *)a)->val * ((Int *)b)->val);
  case FLOAT_TYPE:
    return __float__(((Float *)a)->val * ((Float *)b)->val);
  default:
    return __NaN__();
  }
}

void *__div__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __int__(((Int *)a)->val / ((Int *)b)->val);
  case FLOAT_TYPE:
    return __float__(((Float *)a)->val / ((Float *)b)->val);
  default:
    return __NaN__();
  }
}

void *__eq__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __bool__(((Int *)a)->val == ((Int *)b)->val);
  case FLOAT_TYPE:
    return __bool__(((Float *)a)->val == ((Float *)b)->val);
  default:
    return __NaN__();
  }
}

void *__comp__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __bool__(((Int *)a)->val > ((Int *)b)->val);
  case FLOAT_TYPE:
    return __bool__(((Float *)a)->val > ((Float *)b)->val);
  default:
    return __NaN__();
  }
}

void *__ecomp__(void *a, void *b) {
  __conv__(&a, &b);
  char a_ty = ((Obj *)a)->ty;
  switch (a_ty) {
  case INT_TYPE:
    return __bool__(((Int *)a)->val >= ((Int *)b)->val);
  case FLOAT_TYPE:
    return __bool__(((Float *)a)->val >= ((Float *)b)->val);
  default:
    return __NaN__();
  }
}
Str *__stradd__(Str *a, Str *b) {
  int len = a->len + b->len;
  char *str = (char *)malloc(len);
  memcpy(str, a->val, a->len);
  memcpy(str + a->len, b->val, b->len);

  Str *s = (Str *)malloc(sizeof(Str));
  s->ty = STR_TYPE;
  s->val = str;
  s->len = len;
  return s;
}

void err(char *err, int code) {
  printf("covalent runtime error: %s", err);
  exit(code);
}
