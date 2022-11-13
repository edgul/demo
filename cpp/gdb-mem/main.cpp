#include "Object.h"

void basic_double_delete() {
  Object *o2 = new Object();

  // delete o1; // can't -- delete takes pointer
  delete o2;

  // ./a.out:
  // free(): double free detected in tcache 2
  // Aboted (core dumped)
  // gdb ./a.out:
  // Program received signal SIGABORT, Aborted
  delete o2;
}

Object *dangerous_ptr() { return new Object(); }

template <class T> void delete_helper(T *thing) { delete thing; }

int main() {

  Object *o1 = dangerous_ptr();
  o1->x = 5;
  o1->dd_outer();

  delete_helper(o1);
  delete o1;

  return 0;
}
