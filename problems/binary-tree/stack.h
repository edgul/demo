#pragma once
#include <vector>
#include "binary-tree.h"
#include <optional>

class Stack {
public:
  void push(int n) {
    s.push_back(n);
  }
  std::optional<int> pop() { 
    if (s.empty()) { return std::nullopt; }
    auto n = s[s.size()-1];
    s.pop_back();
    return n;
  }
  std::optional<int> top() {
    if (s.empty()) { return std::nullopt; }
    return s[s.size()-1];
  }
  bool empty() { return s.size()==0;}
  std::vector<int> s; 
};

void testStack() {
  Stack s;
  assert(!s.pop().has_value());
  s.push(3);
  assert(s.top().value() == 3);
  assert(s.pop().value() == 3);
}
