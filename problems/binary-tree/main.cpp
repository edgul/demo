#include <string>
#include "binary-tree.h"
#include "helpers.h"
#include "io.h"
#include "stack.h"

void testConvertStackToTree() {
  Stack s;
  s.push(1);
  s.push(2);
  s.push(3);
  Node* n = convertStackToTree(s);
  assert(n->value == 1);
  assert(n->left->value == 2);
  assert(n->right == nullptr);
  assert(n->left->left->value == 3);
  assert(n->left->right == nullptr);
  assert(n->left->left->right == nullptr);
  assert(!isBalancedTree(n));
}

void runTests() {
  testBalanced(); 
  testStack();
  testConvertStackToTree();
  testReadFromTrees();
  testReadFromTrees1();
  testReadFromTrees2();
}

int main() {
  runTests();
  return 0;
}
