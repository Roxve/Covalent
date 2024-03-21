typedef struct {
  char ty;
} Obj;
typedef struct Int {
  char ty;
  int val;
} Int;

typedef struct Float {
  char ty;
  float val;
} Float;
void writeln(void* args, ...);
void* __int__(int i);
void* __float__(float f);
