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
void writeln(void* arg);
void* __int__(int i);
void* __float__(float f);
void err(char* err, int code);
void* __add__(void *a, void *b);
