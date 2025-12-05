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

