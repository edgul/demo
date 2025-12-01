#include <stdio.h>

extern "C" void hello_dyn();
extern "C" void hello_stat();

int main() {
  printf("hello world\n");
  hello_dyn();
  hello_stat();
  return 0;
}
