#include <string>
#include "binary-tree.h"
#include "helpers.h"
#include "io.h"
#include "stack.h"

void runTests() {
  testBalanced(); 
  testStack();
  testConvertStackToTree();
  testReadFromTrees();
  testReadFromTrees1();
  testReadFromTrees2();
  testReadFromTrees3();
}

int main() {
  runTests();
  return 0;
}
