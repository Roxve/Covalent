
# Covalent
<p align="center">
<img src="img/1.png" alt="logo" width="200"/>
</p>

WIP toy programming language, main purposes is to expirment with "Generic" programming style where everything is generic by default, and to get better at programming

for example this code
```
fun factorial(x) {
  if x == 1 {    
    ret 1
  }
  
  ret x * factorial(x - 1)
}

const x = factorial(5)
const y = factorial(5.0)
```

produces this C code
```c
int factorial$int(int x) {
  if (x == 1) {
    return 1;
  }

  return x - factorial$int(x - 1);
}

float factorial$float(float x) {
  if (x == 1.0) {
    return 1.0;
  }

  return x - factorial$float(x - 1.0);
}

void main() {
  int x = factorial$int(5);
  float y = factorial$float(5.0);
}
```

there has been a bad implentation before where something like this worked but now i will reset the git history back to zero and rewrite from scratch
NOTE that this is not a serious project
