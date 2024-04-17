#! /bin/bash

if test -f gc.o; then
  echo "File exists."
else
  wd=$(pwd)
  cd /tmp/
  git clone https://github.com/ivmai/bdwgc
  cd bdwgc
  gcc -I include -DUSE_MMAP_ANON -DUSE_MUNMAP -c extra/gc.c
  cp gc.o $wd
  cd ..
  rm -rf bdwgc
fi

