#include "stdio.h"
#include <stdlib.h>
#include "std.h"

#define INT_TYPE 0
#define FLOAT_TYPE 1
void writeln(void* arg, ...) {
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


int main() {
  writeln(__int__(5));
  writeln(__float__(3.14));
}
