#include <stdio.h>

// compile libs first with:
// cargo build --release

// compile just dynamic lib:
// g++ main.cpp -Ldynlib/target/release -ldynlib -o a.out

// compile just static lib:
// g++ main.cpp statlib/target/release/libstatlib.a -o a.out

// compile main.cpp, link the static and dynamic libs
// g++ main.cpp statlib/target/release/libstatlib.a -Ldynlib/target/release -ldynlib -o a.out

// tell the loader where to look for dynamic libs (before running):
// export LD_LIBRARY_PATH=./dynlib/target/release:$LD_LIBRARY_PATH  # Linux

extern "C" void hello_dyn();
extern "C" void hello_stat();

int main() {
  printf("hello world\n");
  hello_dyn();
  hello_stat();
  return 0;
}
