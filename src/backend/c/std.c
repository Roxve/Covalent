#include "stdio.h"
#include <stdlib.h>
#include "std.h"

#define INT_TYPE 0
#define FLOAT_TYPE 1 

void writeln(void* arg) {
  char ty = ((Obj*) arg)->ty;
  
  switch(ty) {
    case INT_TYPE:
    {
      Int* i = (Int*) arg;
      printf("%d\n", i->val);
      break;
    }

    case FLOAT_TYPE:
    {
      Float* f = (Float*) arg;
      printf("%f\n", f->val);
    }
  }
}



void* __int__(int i) {
  Int* obj = (Int*)malloc(sizeof(Int));
  obj->ty = INT_TYPE;
  obj->val = i;
  return obj;
}
void* __float__(float f) {
  Float* obj = (Float*)malloc(sizeof(Float));
  obj->ty = FLOAT_TYPE;
  obj->val = f;
  return obj;
}


void* __add__(void *a, void *b) {
  char a_ty = ((Obj*) a)->ty;
  char b_ty = ((Obj*) b)->ty;
  if(a_ty != b_ty) {
    err("cannot add a, and b", 5);
  }

  switch(a_ty) {
    case INT_TYPE:
      return __int__(((Int*) a)->val + ((Int*) b)->val);
  }
}

void err(char* err, int code) {
  printf("covalent runtime error: %s", err);
  exit(code);
}