#pragma once
#include "binary-tree.h"
#include "stack.h"

// this will build an unbalanced tree
Node* convertStackToTree(Stack s) {
  if (s.empty()) { return nullptr; }
  Node* last = nullptr;
  while (s.top().has_value()) {
    Node* current = new Node(s.pop().value());
    current->left = last; // link the last item to the current item
    last = current;
  }
  return last; // last is root node
}

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
