#pragma once

#include <algorithm>
#include <assert.h>
#include <stdio.h>
#include <string>

// Node for a Binary Tree
class Node {
public:
  int value;
  Node* left; // todo: make this safer by getting rid of raw pointers?
  Node* right;
  Node(int val) : value(val) {}
  Node(int val, Node* left, Node* right) : value(val), left(left), right(right) {}
};

bool diffGreaterThanOne(int lhs, int rhs) { return std::abs(lhs-rhs) > 1;}

// todo: this feels really clunky, is there a better way?
struct MaxAndMin {
  int max;
  int min;
  MaxAndMin(int max, int min) : max(max), min(min) {}
  bool balanced() { return !diffGreaterThanOne(max, min); }
};

MaxAndMin isBalancedHelper(Node* node) {
  if (!node) {
    return MaxAndMin(0,0);
  }
  auto left = isBalancedHelper(node->left);
  if (!left.balanced()) {
    // no need to evaluate other arms if we already know we aren't balanced
    // note that this assumes caller code only needs min/max for the purpose of
    // balance calculation
    return left;
  } 
  auto right = isBalancedHelper(node->right);
  if (!right.balanced()) {
    return right; // no need again here
  } 

  auto min = std::min(left.min, right.min);
  auto max = std::max(left.max, right.max);
  return MaxAndMin(max+1,min+1);
}

bool isBalancedTree(Node* root) {
  auto maxAndMin = isBalancedHelper(root);
  if (!maxAndMin.balanced()) { return false; }
  if (!diffGreaterThanOne(maxAndMin.max, maxAndMin.min)) {
    return true;
  }
  return false;
}

std::string serialize(Node* node) {
  if (!node) { return ""; }
  std::string result = std::to_string(node->value) + "{";
  // todo: maybe some perf concerns around appending strings?
  result.append(serialize(node->left));
  result.append(",");
  result.append(serialize(node->right));
  result.append("}");
  return result;
}

enum Mode {
  INIT,
  VALUE,
  LEFT,
  RIGHT,
};

Node* deserialize(std::string& string, size_t& index) {
  Node* node = nullptr;
  std::string digits("");
  Mode m = INIT;
  while (index < string.length()) {
    char c = string[index];
    if (m == INIT || m == VALUE) {
      if (isdigit(c)) {
        digits += c;   
        index++;
        m = VALUE;
        continue;
      } else if (c == '{')  {
        m = LEFT;
        index++;
        node = new Node(std::stoi(digits));
        node->left = nullptr;
        node->right = nullptr;
      } else if (c == '\n' || c == ',' || c == '}'){
        // missing {} means we can cut out early
        node = new Node(std::stoi(digits));
        node->left = nullptr;
        node->right = nullptr;
        break;
      } else {
        assert(false && "unexpected char while looking for value");
      }
    } else if (m == LEFT) {
      if (isdigit(c)) {
        node->left = deserialize(string, index);
        continue;
      } else if (c == ',') {
        index++;
        m = RIGHT;  
      } else if (c == '}') { // no comma implies no children, lets cut out early
        index++;
        break;
      } else {
        assert(false && "unexpected char while looking for left");
      }
    } else if (m == RIGHT) {
      if (isdigit(c)) {
        node->right = deserialize(string, index);
        char new_c = string[index];
        assert(new_c == '}' && "expected closing paren after right is processed");
        index++; // slightly earlier than taking the next branch after iteration
      } else if (c == '}') {
        // empty right, no need to recurse, just increment counter and move on
        index++;
      } else {
        assert(false && "unexpected char while looking for right");
      }
      break; // we are done in this recursion
    }
  }
  // single number (no braces) edge case:
  // if we fell off the array with some unused digits
  if (m == VALUE && !digits.empty()) {
    node = new Node(std::stoi(digits));
    node->left = nullptr;
    node->right = nullptr;
  }
  return node;
}

// todo: more stuff with trees themselves
// bool isBinarySearchTree(Node* root) { }

void testBalanced() {
  Node* empty = nullptr;
  assert(isBalancedTree(empty));

  Node* singleNode = new Node(1);
  assert(isBalancedTree(singleNode));

  Node* triangle = new Node(1, new Node(2), new Node(3));
  assert(isBalancedTree(triangle));

  Node* leftLeg = new Node(1, new Node(2, new Node(3), nullptr), nullptr);
  assert(!isBalancedTree(leftLeg));

  Node* rightLeg = new Node(1, nullptr, new Node(2, nullptr, new Node(3)));
  assert(!isBalancedTree(rightLeg));
}
